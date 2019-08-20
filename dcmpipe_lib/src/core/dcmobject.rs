use crate::core::dcmelement::DicomElement;
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::io::{Error, ErrorKind};

pub struct DicomObject {
    /// For root dataset this will be `None`, for sequence elements this will be the sequence
    /// element itself.
    element: Option<DicomElement>,
    /// Child nodes which may be elements of sub-sequences
    child_nodes: BTreeMap<u32, DicomObject>,
}

impl DicomObject {
    pub fn new_root() -> DicomObject {
        DicomObject {
            element: None,
            child_nodes: BTreeMap::new(),
        }
    }

    pub fn new_with_element(element: DicomElement) -> DicomObject {
        DicomObject {
            element: Some(element),
            child_nodes: BTreeMap::new(),
        }
    }

    pub fn as_element(&self) -> Option<&DicomElement> {
        self.element.as_ref()
    }

    pub fn get_object_count(&self) -> usize {
        self.child_nodes.len()
    }

    pub fn get_object(&self, tag: u32) -> Option<&DicomObject> {
        self.child_nodes.get(&tag)
    }

    pub fn put_object(&mut self, object: DicomObject) -> Result<Option<DicomObject>, Error> {
        let tag: u32;
        if let Some(element) = object.as_element() {
            tag = element.tag;
        } else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Attempting to insert root node as child",
            ));
        }
        Ok(self.child_nodes.insert(tag, object))
    }

    pub fn iter(&self) -> Iter<u32, DicomObject> {
        self.child_nodes.iter()
    }
}