//! This is an auto-generated file. Do not make modifications here.

use core::tag::Tag;
use core::vm::VM;
use core::vr;
/// File-set ID
/// 
/// - **Tag:** (0004,1130)
/// - **VR:** CS
/// - **VM:** 1
pub static FileSetID: Tag = Tag {
    ident: "FileSetID",
    tag: "(0004,1130)",
    implicit_vr: Some(&vr::CS),
    vm: &VM::Distinct(1),
    desc: "File-set ID",
};

/// File-set Descriptor File ID
/// 
/// - **Tag:** (0004,1141)
/// - **VR:** CS
/// - **VM:** 1-8
pub static FileSetDescriptorFileID: Tag = Tag {
    ident: "FileSetDescriptorFileID",
    tag: "(0004,1141)",
    implicit_vr: Some(&vr::CS),
    vm: &VM::AtMost(8),
    desc: "File-set Descriptor File ID",
};

/// Specific Character Set of File-set Descriptor File
/// 
/// - **Tag:** (0004,1142)
/// - **VR:** CS
/// - **VM:** 1
pub static SpecificCharacterSetOfFileSetDescriptorFile: Tag = Tag {
    ident: "SpecificCharacterSetOfFileSetDescriptorFile",
    tag: "(0004,1142)",
    implicit_vr: Some(&vr::CS),
    vm: &VM::Distinct(1),
    desc: "Specific Character Set of File-set Descriptor File",
};

/// Offset of the First Directory Record of the Root Directory Entity
/// 
/// - **Tag:** (0004,1200)
/// - **VR:** UL
/// - **VM:** 1
pub static OffsetOfTheFirstDirectoryRecordOfTheRootDirectoryEntity: Tag = Tag {
    ident: "OffsetOfTheFirstDirectoryRecordOfTheRootDirectoryEntity",
    tag: "(0004,1200)",
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "Offset of the First Directory Record of the Root Directory Entity",
};

/// Offset of the Last Directory Record of the Root Directory Entity
/// 
/// - **Tag:** (0004,1202)
/// - **VR:** UL
/// - **VM:** 1
pub static OffsetOfTheLastDirectoryRecordOfTheRootDirectoryEntity: Tag = Tag {
    ident: "OffsetOfTheLastDirectoryRecordOfTheRootDirectoryEntity",
    tag: "(0004,1202)",
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "Offset of the Last Directory Record of the Root Directory Entity",
};

/// File-set Consistency Flag
/// 
/// - **Tag:** (0004,1212)
/// - **VR:** US
/// - **VM:** 1
pub static FileSetConsistencyFlag: Tag = Tag {
    ident: "FileSetConsistencyFlag",
    tag: "(0004,1212)",
    implicit_vr: Some(&vr::US),
    vm: &VM::Distinct(1),
    desc: "File-set Consistency Flag",
};

/// Directory Record Sequence
/// 
/// - **Tag:** (0004,1220)
/// - **VR:** SQ
/// - **VM:** 1
pub static DirectoryRecordSequence: Tag = Tag {
    ident: "DirectoryRecordSequence",
    tag: "(0004,1220)",
    implicit_vr: Some(&vr::SQ),
    vm: &VM::Distinct(1),
    desc: "Directory Record Sequence",
};

/// Offset of the Next Directory Record
/// 
/// - **Tag:** (0004,1400)
/// - **VR:** UL
/// - **VM:** 1
pub static OffsetOfTheNextDirectoryRecord: Tag = Tag {
    ident: "OffsetOfTheNextDirectoryRecord",
    tag: "(0004,1400)",
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "Offset of the Next Directory Record",
};

/// Record In-use Flag
/// 
/// - **Tag:** (0004,1410)
/// - **VR:** US
/// - **VM:** 1
pub static RecordInUseFlag: Tag = Tag {
    ident: "RecordInUseFlag",
    tag: "(0004,1410)",
    implicit_vr: Some(&vr::US),
    vm: &VM::Distinct(1),
    desc: "Record In-use Flag",
};

/// Offset of Referenced Lower-Level Directory Entity
/// 
/// - **Tag:** (0004,1420)
/// - **VR:** UL
/// - **VM:** 1
pub static OffsetOfReferencedLowerLevelDirectoryEntity: Tag = Tag {
    ident: "OffsetOfReferencedLowerLevelDirectoryEntity",
    tag: "(0004,1420)",
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "Offset of Referenced Lower-Level Directory Entity",
};

/// Directory Record Type
/// 
/// - **Tag:** (0004,1430)
/// - **VR:** CS
/// - **VM:** 1
pub static DirectoryRecordType: Tag = Tag {
    ident: "DirectoryRecordType",
    tag: "(0004,1430)",
    implicit_vr: Some(&vr::CS),
    vm: &VM::Distinct(1),
    desc: "Directory Record Type",
};

/// Private Record UID
/// 
/// - **Tag:** (0004,1432)
/// - **VR:** UI
/// - **VM:** 1
pub static PrivateRecordUID: Tag = Tag {
    ident: "PrivateRecordUID",
    tag: "(0004,1432)",
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Private Record UID",
};

/// Referenced File ID
/// 
/// - **Tag:** (0004,1500)
/// - **VR:** CS
/// - **VM:** 1-8
pub static ReferencedFileID: Tag = Tag {
    ident: "ReferencedFileID",
    tag: "(0004,1500)",
    implicit_vr: Some(&vr::CS),
    vm: &VM::AtMost(8),
    desc: "Referenced File ID",
};

/// MRDR Directory Record Offset
/// 
/// - **Tag:** (0004,1504)
/// - **VR:** UL
/// - **VM:** 1
pub static MRDRDirectoryRecordOffset: Tag = Tag {
    ident: "MRDRDirectoryRecordOffset",
    tag: "(0004,1504)",
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "MRDR Directory Record Offset",
};

/// Referenced SOP Class UID in File
/// 
/// - **Tag:** (0004,1510)
/// - **VR:** UI
/// - **VM:** 1
pub static ReferencedSOPClassUIDInFile: Tag = Tag {
    ident: "ReferencedSOPClassUIDInFile",
    tag: "(0004,1510)",
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Referenced SOP Class UID in File",
};

/// Referenced SOP Instance UID in File
/// 
/// - **Tag:** (0004,1511)
/// - **VR:** UI
/// - **VM:** 1
pub static ReferencedSOPInstanceUIDInFile: Tag = Tag {
    ident: "ReferencedSOPInstanceUIDInFile",
    tag: "(0004,1511)",
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Referenced SOP Instance UID in File",
};

/// Referenced Transfer Syntax UID in File
/// 
/// - **Tag:** (0004,1512)
/// - **VR:** UI
/// - **VM:** 1
pub static ReferencedTransferSyntaxUIDInFile: Tag = Tag {
    ident: "ReferencedTransferSyntaxUIDInFile",
    tag: "(0004,1512)",
    implicit_vr: Some(&vr::UI),
    vm: &VM::Distinct(1),
    desc: "Referenced Transfer Syntax UID in File",
};

/// Referenced Related General SOP Class UID in File
/// 
/// - **Tag:** (0004,151A)
/// - **VR:** UI
/// - **VM:** 1-n
pub static ReferencedRelatedGeneralSOPClassUIDInFile: Tag = Tag {
    ident: "ReferencedRelatedGeneralSOPClassUIDInFile",
    tag: "(0004,151A)",
    implicit_vr: Some(&vr::UI),
    vm: &VM::AtLeast(1),
    desc: "Referenced Related General SOP Class UID in File",
};

/// Number of References
/// 
/// - **Tag:** (0004,1600)
/// - **VR:** UL
/// - **VM:** 1
pub static NumberOfReferences: Tag = Tag {
    ident: "NumberOfReferences",
    tag: "(0004,1600)",
    implicit_vr: Some(&vr::UL),
    vm: &VM::Distinct(1),
    desc: "Number of References",
};
