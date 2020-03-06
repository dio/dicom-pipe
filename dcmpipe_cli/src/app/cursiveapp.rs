use crate::app::render_value;
use cursive::traits::{Boxable, Identifiable};
use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use cursive_table_view::{TableView, TableViewItem};
use dcmpipe_dict::dict::lookup::TAG_BY_VALUE;
use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmparser::{Parser, ParserBuilder};
use dcmpipe_lib::defn::tag::Tag;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub struct CursiveApp {
    openpath: String,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum DicomElementColumn {
    Expand,
    Tag,
    Name,
    VR,
    Value,
}

#[derive(Clone)]
pub struct DicomElementValue {
    tag: u32,
    seq: String,
    tag_display: String,
    name: String,
    vr: String,
    value: String,
}
impl DicomElementValue {
    fn new(element: DicomElement) -> DicomElementValue {
        let seq: String = if element.is_seq_like() { "+" } else { "" }.to_owned();
        let name: String = if let Some(tag) = TAG_BY_VALUE.get(&element.tag) {
            tag.ident
        } else {
            "<Private Tag>"
        }
        .to_owned();

        let value: String = if let Ok(value) = render_value(&element) {
            value
        } else {
            "<Error Parsing Value>".to_owned()
        };

        DicomElementValue {
            tag: element.tag,
            seq,
            tag_display: Tag::format_tag_to_display(element.tag),
            name,
            vr: element.vr.ident.to_owned(),
            value,
        }
    }

    fn mini_display(&self) -> String {
        format!("{} {} {}", self.tag_display, self.vr, self.name)
    }
}

impl DicomElementColumn {
    fn as_str(&self) -> &str {
        match *self {
            DicomElementColumn::Expand => "+",
            DicomElementColumn::Tag => "Tag",
            DicomElementColumn::Name => "Name",
            DicomElementColumn::VR => "VR",
            DicomElementColumn::Value => "Value",
        }
    }
}

impl TableViewItem<DicomElementColumn> for DicomElementValue {
    fn to_column(&self, column: DicomElementColumn) -> String {
        match column {
            DicomElementColumn::Expand => self.seq.clone(),
            DicomElementColumn::Tag => self.tag_display.clone(),
            DicomElementColumn::Name => self.name.clone(),
            DicomElementColumn::VR => self.vr.clone(),
            DicomElementColumn::Value => self.value.clone(),
        }
    }

    fn cmp(&self, other: &Self, column: DicomElementColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            DicomElementColumn::Expand => Ordering::Equal,
            DicomElementColumn::Tag => self.tag.cmp(&other.tag),
            DicomElementColumn::Name => Ordering::Equal,
            DicomElementColumn::VR => Ordering::Equal,
            DicomElementColumn::Value => Ordering::Equal,
        }
    }
}

impl<'me> CursiveApp {
    pub fn new(openpath: String) -> CursiveApp {
        CursiveApp { openpath }
    }

    pub fn run(&'me mut self) -> Result<(), Error> {
        let path: &Path = Path::new(&self.openpath);

        if !path.is_file() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("invalid file: {}", path.display()),
            ));
        }

        let file: File = File::open(path)?;
        let parser: Parser<'_, File> = ParserBuilder::new(file)
            .dictionary(&STANDARD_DICOM_DICTIONARY)
            .build();

        let mut items: Vec<DicomElementValue> = Vec::new();
        let mut total_name_size: usize = 0;
        for elem in parser {
            let elem: DicomElement = elem?;
            if elem.get_sequence_path().is_empty() {
                let name: String = if let Some(tag) = TAG_BY_VALUE.get(&elem.tag) {
                    tag.ident
                } else {
                    "<Private Tag>"
                }
                .to_owned();

                total_name_size = name.len().max(total_name_size);

                items.push(DicomElementValue::new(elem));
            }
        }

        let mut cursive: Cursive = Cursive::default();
        cursive.add_global_callback('q', Cursive::quit);

        let mut table = TableView::<DicomElementValue, DicomElementColumn>::new()
            .column(
                DicomElementColumn::Expand,
                DicomElementColumn::Expand.as_str(),
                |c| c.width(5),
            )
            .column(
                DicomElementColumn::Tag,
                DicomElementColumn::Tag.as_str(),
                |c| c.width(12).ordering(Ordering::Greater),
            )
            .column(
                DicomElementColumn::Name,
                DicomElementColumn::Name.as_str(),
                |c| c.width(total_name_size),
            )
            .column(
                DicomElementColumn::VR,
                DicomElementColumn::VR.as_str(),
                |c| c.width(5),
            )
            .column(
                DicomElementColumn::Value,
                DicomElementColumn::Value.as_str(),
                |c| c,
            );

        table.set_items(items);

        table.set_on_submit(|siv: &mut Cursive, _row: usize, index: usize| {
            let title: String = siv
                .call_on_name(
                    "table",
                    move |table: &mut TableView<DicomElementValue, DicomElementColumn>| {
                        table.borrow_item(index).unwrap().mini_display()
                    },
                )
                .unwrap();

            let value: String = siv
                .call_on_name(
                    "table",
                    move |table: &mut TableView<DicomElementValue, DicomElementColumn>| {
                        table.borrow_item(index).unwrap().value.clone()
                    },
                )
                .unwrap();

            siv.add_layer(Dialog::around(TextView::new(value)).title(title).button(
                "Close",
                move |s| {
                    s.pop_layer();
                },
            ));
        });

        cursive.add_layer(
            Dialog::around(table.with_name("table").full_screen()).title("DICOM Viewer"),
        );

        cursive.run();

        Ok(())
    }
}
