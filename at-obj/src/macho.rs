//! # Example Mach-O file format
//!
//!     00              08             0F
//!     _________________________________
//!  00 |            Header64           |
//!  10 |_______________________________|
//!  20 |                               |
//!  30 |        Segment64Command       |
//!  40 |                               |
//!  50 |               ________________|
//!  60 |_______________|               |
//!  70 |                               |
//!  80 |           Section64           |
//!  90 |                               |
//!  A0 |               ________________|
//!  B0 |_______________| SymtabCommand |
//!  C0 |_______________________________|
//!  D0 |         SectionData           |
//!  E0 |               ________________|
//!  F0 |_______________|               |
//! 100 |_________RelocationInfo________|
//! 110 |                               |
//! 120 |_________SymbolTable___________|
//! 130 |_________StringTable_______|
use crate::{
    num::NumExt as _,
    object::{BssSection, DataSection, Object, SectionRef, Symbol, TextSection},
};
use at_mac::{
    header::{CpuSubTypeX86_64, CpuType, FileType, Flags, Header64, Magic},
    load_command::{
        segment64::{Section64, SectionAttr, SectionAttrs, SectionType, SegmentCommand64},
        symtab::SymtabCommand,
    },
    nlist::{NList64, NType, NTypeField},
    reloc::{RelocLength, RelocationInfo, X86_64RelocType},
    string_table::StringTable,
};
use std::io::Write;

/// ObjectをMach-O形式で書き込む
pub fn write_into<W: Write>(object: &Object, write: &mut W) {
    // write Header64
    gen_header64(object).write_into(write);

    // write SegmentCommand64
    gen_segment_command64(object).write_into(write);

    // write Section64
    let sections = gen_section64s(object);
    sections.iter().for_each(|sect| sect.write_into(write));

    // write SymtabCommand
    gen_symtab_command(object).write_into(write);

    // write SectionData
    write_section_data_into(object, write);

    // create StringTable (write later)
    let stab = gen_string_table(object);

    // create Vec<NList64> (write later)
    let symbols = gen_nlist64s(object, &sections, &stab);

    // write Vec<RelocationInfo>
    gen_relocation_infos(object, &symbols, &stab)
        .iter()
        .for_each(|reloc| reloc.write_into(write));

    // write Vec<NList64>
    symbols.iter().for_each(|sym| sym.write_into(write));

    // write StringTable
    write.write_all(stab.as_ref()).unwrap();
}

/// object形式の `Header64` を生成する.
/// 現状、x86-64限定.
fn gen_header64(object: &Object) -> Header64 {
    Header64 {
        magic: Magic::Magic64,
        cpu_type: CpuType::X86_64(CpuSubTypeX86_64::All),
        file_type: FileType::Object,
        n_cmds: 2,
        size_of_cmds: SegmentCommand64::SIZE
            + object.sections().len() * Section64::SIZE
            + SymtabCommand::SIZE,
        flags: Flags::new(),
        reserved: 0,
    }
}

fn gen_segment_command64(object: &Object) -> SegmentCommand64 {
    SegmentCommand64 {
        cmd: SegmentCommand64::TYPE,
        cmdsize: SegmentCommand64::SIZE + object.sections().len() * Section64::SIZE,
        // object fileのsegnameは常に空文字
        segname: "".to_string(),
        vmaddr: 0,
        vmsize: object.sections().iter().map(|sect| sect.vm_size()).sum(),
        fileoff: (Header64::SIZE
            + SegmentCommand64::SIZE
            + object.sections().len() * Section64::SIZE
            + SymtabCommand::SIZE) as u64,
        filesize: object
            .sections()
            .iter()
            .map(|sect| sect.file_size())
            .sum::<u32>()
            .aligned(8) as u64,
        // object fileのprotectionは常に7
        // つまりrwxの全てのbitが立っている状態
        maxprot: 7,
        initprot: 7,
        nsects: object.sections().len(),
        flags: 0,
    }
}

fn gen_section64s(object: &Object) -> Vec<Section64> {
    let mut vmaddr = 0_u64;
    let mut data_start = Header64::SIZE
        + SegmentCommand64::SIZE
        + object.sections().len() * Section64::SIZE
        + SymtabCommand::SIZE;
    let mut reloc_start = data_start
        + object
            .sections()
            .iter()
            .map(|sect| sect.file_size())
            .sum::<u32>()
            .aligned(8);

    object
        .sections()
        .iter()
        .map(|section| {
            let addr = vmaddr;
            vmaddr += section.vm_size();

            let offset = data_start;
            data_start += section.file_size();

            let reloff = reloc_start;
            reloc_start += RelocationInfo::SIZE * section.relocs().len() as u32;

            gen_section64(section, addr, offset, reloff)
        })
        .collect()
}

fn gen_section64<'a>(section: SectionRef<'a>, addr: u64, offset: u32, reloff: u32) -> Section64 {
    use SectionRef::*;

    match section {
        Text(text) => gen_section64_from_text(text, addr, offset, reloff),
        Data(data) => gen_section64_from_data(data, addr, offset, reloff),
        Bss(bss) => gen_section64_from_bss(bss, addr),
    }
}

fn gen_section64_from_text(text: &TextSection, addr: u64, offset: u32, reloff: u32) -> Section64 {
    let mut attrs = SectionAttrs::new();
    attrs.push(SectionAttr::SomeInstructions);
    attrs.push(SectionAttr::PureInstructions);
    if !text.relocs.is_empty() {
        attrs.push(SectionAttr::LocReloc);
        attrs.push(SectionAttr::ExtReloc);
    }

    Section64 {
        sectname: "__text".to_string(),
        segname: "__TEXT".to_string(),
        addr,
        size: text.bytes.len() as u64,
        offset,
        align: 0,
        reloff,
        nreloc: text.relocs.len() as u32,
        flags: (attrs, SectionType::Regular),
        reserved1: 0,
        reserved2: 0,
        reserved3: 0,
    }
}

fn gen_section64_from_data(data: &DataSection, addr: u64, offset: u32, reloff: u32) -> Section64 {
    let mut attrs = SectionAttrs::new();
    if !data.relocs.is_empty() {
        attrs.push(SectionAttr::LocReloc);
        attrs.push(SectionAttr::ExtReloc);
    }

    Section64 {
        sectname: "__data".to_string(),
        segname: "__DATA".to_string(),
        addr,
        size: data.bytes.len() as u64,
        offset,
        align: 0,
        reloff,
        nreloc: data.relocs.len() as u32,
        flags: (attrs, SectionType::Regular),
        reserved1: 0,
        reserved2: 0,
        reserved3: 0,
    }
}

fn gen_section64_from_bss(bss: &BssSection, addr: u64) -> Section64 {
    Section64 {
        sectname: "__bss".to_string(),
        segname: "__DATA".to_string(),
        addr,
        size: bss.size,
        offset: 0,
        align: 0,
        reloff: 0,
        nreloc: 0,
        flags: (SectionAttrs::new(), SectionType::Zerofill),
        reserved1: 0,
        reserved2: 0,
        reserved3: 0,
    }
}

fn gen_symtab_command(object: &Object) -> SymtabCommand {
    let symoff = Header64::SIZE
        + SegmentCommand64::SIZE
        + object.sections().len() * Section64::SIZE
        + SymtabCommand::SIZE
        + object
            .sections()
            .iter()
            .map(|s| s.file_size())
            .sum::<u32>()
            .aligned(8)
        + object
            .sections()
            .iter()
            .map(|s| s.relocs().len() as u32)
            .sum::<u32>()
            * RelocationInfo::SIZE;
    let nsyms = object
        .sections()
        .iter()
        .map(|s| s.symbols().len() as u32)
        .sum::<u32>();
    let stroff = symoff + nsyms * NList64::SIZE;
    let strsize = object
        .sections()
        .iter()
        .map(|s| {
            s.symbols()
                .iter()
                .map(|sym| sym.name().len() as u32 + 1)
                .sum::<u32>()
        })
        .sum::<u32>()
        + 1;

    SymtabCommand {
        cmd: SymtabCommand::TYPE,
        cmdsize: SymtabCommand::SIZE,
        symoff,
        nsyms,
        stroff,
        strsize,
    }
}

fn write_section_data_into<W: Write>(object: &Object, write: &mut W) {
    let mut file_size = 0;
    object.sections().iter().for_each(|sect| {
        write.write_all(sect.file_data()).unwrap();
        file_size += sect.file_size();
    });
    let padding = [0u8; 7];
    let n_padding = file_size.padding(8) as usize;
    write.write_all(&padding[..n_padding]).unwrap();
}

fn gen_string_table(object: &Object) -> StringTable {
    let mut stab = StringTable::with_null();
    object.sections().iter().for_each(|sect| {
        sect.symbols()
            .iter()
            .for_each(|sym| stab.push_with_null(sym.name()))
    });
    stab
}

fn gen_nlist64s(object: &Object, sections: &Vec<Section64>, stab: &StringTable) -> Vec<NList64> {
    fn get_strx(stab: &StringTable, name: &str) -> u32 {
        let mut idx = 0;
        for s in stab.iter() {
            idx += 1;
            if s == name {
                break;
            } else {
                idx += s.len() as u32;
            }
        }
        idx
    }

    let mut nlists = Vec::new();

    object.sections().iter().enumerate().for_each(|(i, sect)| {
        let idx = i + 1;
        sect.symbols().iter().for_each(|sym| {
            nlists.push(match sym {
                Symbol::Undef { name } => NList64 {
                    n_strx: get_strx(stab, name.as_str()),
                    n_type: NTypeField::Norm {
                        n_pext: false,
                        n_type: NType::Undf,
                        n_ext: true,
                    },
                    n_sect: 0,
                    n_desc: 0,
                    n_value: 0,
                },
                Symbol::Abs { name, val, ext } => NList64 {
                    n_strx: get_strx(stab, name.as_str()),
                    n_type: NTypeField::Norm {
                        n_pext: false,
                        n_type: NType::Abs,
                        n_ext: *ext,
                    },
                    n_sect: idx as u8,
                    n_desc: 0,
                    n_value: *val,
                },
                Symbol::Ref { name, addr, ext } => NList64 {
                    n_strx: get_strx(stab, name.as_str()),
                    n_type: NTypeField::Norm {
                        n_pext: false,
                        n_type: NType::Sect,
                        n_ext: *ext,
                    },
                    n_sect: idx as u8,
                    n_desc: 0,
                    n_value: sections[idx - 1].addr + *addr,
                },
            })
        });
    });

    nlists
}

fn gen_relocation_infos(
    object: &Object,
    symbols: &[NList64],
    stab: &StringTable,
) -> Vec<RelocationInfo> {
    fn get_sym_idx(symbols: &[NList64], stab: &StringTable, name: &str) -> u32 {
        symbols
            .iter()
            .enumerate()
            .find(|(_, sym)| stab.get(sym.n_strx as usize) == name)
            .map(|(idx, _)| idx as u32)
            .unwrap()
    }

    let mut reloc_infos = Vec::new();

    object.sections().iter().for_each(|sect| {
        sect.relocs().iter().for_each(|reloc| {
            let reloc_info = RelocationInfo {
                r_address: reloc.addr,
                r_symbolnum: get_sym_idx(symbols, stab, reloc.symbol.as_str()),
                r_pcrel: reloc.pcrel,
                r_length: RelocLength::from_u32(reloc.len as u32),
                r_extern: true,
                // TODO
                // 適切に設定できるようにする
                r_type: X86_64RelocType::Unsigned.to_u8(),
            };
            reloc_infos.push(reloc_info);
        });
    });

    reloc_infos
}
