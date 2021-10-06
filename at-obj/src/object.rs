pub struct Object {
    pub sections: Sections,
}

impl Object {
    pub fn new() -> Self {
        Object {
            sections: Sections {
                text: TextSection::new(),
                data: DataSection::new(),
                bss: BssSection::new(),
            },
        }
    }

    pub fn sections(&self) -> &Sections {
        &self.sections
    }

    pub fn symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.sections
            .text
            .symbols
            .iter()
            .chain(self.sections.data.symbols.iter())
            .chain(self.sections.bss.symbols.iter())
    }

    pub fn symbols_mut(&mut self) -> impl Iterator<Item = &mut Symbol> {
        self.sections
            .text
            .symbols
            .iter_mut()
            .chain(self.sections.data.symbols.iter_mut())
            .chain(self.sections.bss.symbols.iter_mut())
    }
}

pub struct Sections {
    pub text: TextSection,
    pub data: DataSection,
    pub bss: BssSection,
}

impl Sections {
    /// iterate of all sections except empty section.
    pub(crate) fn iter<'a>(&'a self) -> impl Iterator<Item = SectionRef<'a>> {
        let arr = [
            SectionRef::Text(&self.text),
            SectionRef::Data(&self.data),
            SectionRef::Bss(&self.bss),
        ];
        std::array::IntoIter::new(arr).filter(|sect| !sect.is_empty())
    }

    pub(crate) fn len(&self) -> u32 {
        self.iter().count() as u32
    }
}

pub(crate) enum SectionRef<'a> {
    Text(&'a TextSection),
    Data(&'a DataSection),
    Bss(&'a BssSection),
}

impl<'a> SectionRef<'a> {
    pub(crate) fn vm_size(&self) -> u64 {
        use SectionRef::*;

        match self {
            Text(text) => text.bytes.len() as u64,
            Data(data) => data.bytes.len() as u64,
            Bss(bss) => bss.size,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.vm_size() == 0
    }

    pub(crate) fn file_data(&self) -> &[u8] {
        use SectionRef::*;

        const EMPTY: [u8; 0] = [];

        match self {
            Text(text) => text.bytes.as_slice(),
            Data(data) => data.bytes.as_slice(),
            Bss(_) => &EMPTY,
        }
    }

    pub(crate) fn file_size(&self) -> u32 {
        self.file_data().len() as u32
    }

    pub(crate) fn symbols(&self) -> &[Symbol] {
        use SectionRef::*;

        match self {
            Text(text) => text.symbols.as_slice(),
            Data(data) => data.symbols.as_slice(),
            Bss(bss) => bss.symbols.as_slice(),
        }
    }

    pub(crate) fn relocs(&self) -> &[Reloc] {
        use SectionRef::*;

        const EMPTY: [Reloc; 0] = [];

        match self {
            Text(text) => text.relocs.as_slice(),
            Data(data) => data.relocs.as_slice(),
            Bss(_) => &EMPTY,
        }
    }
}

pub(crate) trait Section {
    fn vm_size(&self) -> u64;
    fn file_data(&self) -> &[u8];
    fn symbols(&self) -> &[Symbol];
    fn relocs(&self) -> &[Reloc];

    fn file_size(&self) -> u32 {
        self.file_data().len() as u32
    }
}

pub struct TextSection {
    pub bytes: Vec<u8>,
    pub symbols: Vec<Symbol>,
    pub relocs: Vec<Reloc>,
}

impl TextSection {
    pub fn new() -> TextSection {
        TextSection {
            bytes: Vec::new(),
            symbols: Vec::new(),
            relocs: Vec::new(),
        }
    }
}

pub struct DataSection {
    pub bytes: Vec<u8>,
    pub symbols: Vec<Symbol>,
    pub relocs: Vec<Reloc>,
}

impl DataSection {
    pub fn new() -> DataSection {
        DataSection {
            bytes: Vec::new(),
            symbols: Vec::new(),
            relocs: Vec::new(),
        }
    }
}

pub struct BssSection {
    pub size: u64,
    pub symbols: Vec<Symbol>,
}

impl BssSection {
    pub fn new() -> Self {
        BssSection {
            size: 0,
            symbols: Vec::new(),
        }
    }
}

pub struct Reloc {
    /// offset from the start of the section to the
    /// item containing the address requiring relocation
    pub addr: i32,
    pub symbol: String,
    pub pcrel: bool,
    // 0 => 1 byte, 1 => 2 byte, 2 => 4 byte, 3 => 8 byte
    pub len: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    Undef { name: String },
    Abs { name: String, val: u64, ext: bool },
    Ref { name: String, addr: u64, ext: bool },
}

impl Symbol {
    pub fn name(&self) -> &str {
        match self {
            Symbol::Undef { name } => name.as_str(),
            Symbol::Abs { name, .. } => name.as_str(),
            Symbol::Ref { name, .. } => name.as_str(),
        }
    }
}
