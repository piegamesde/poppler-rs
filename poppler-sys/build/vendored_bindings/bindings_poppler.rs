/* automatically generated by rust-bindgen */

pub type guint16 = ::std::os::raw::c_ushort;
extern "C" {
    pub fn poppler_error_quark() -> GQuark;
}
pub const PopplerError_POPPLER_ERROR_INVALID: PopplerError = 0;
pub const PopplerError_POPPLER_ERROR_ENCRYPTED: PopplerError = 1;
pub const PopplerError_POPPLER_ERROR_OPEN_FILE: PopplerError = 2;
pub const PopplerError_POPPLER_ERROR_BAD_CATALOG: PopplerError = 3;
pub const PopplerError_POPPLER_ERROR_DAMAGED: PopplerError = 4;
pub type PopplerError = u32;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_REPLACE: PopplerPageTransitionType = 0;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_SPLIT: PopplerPageTransitionType = 1;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_BLINDS: PopplerPageTransitionType = 2;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_BOX: PopplerPageTransitionType = 3;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_WIPE: PopplerPageTransitionType = 4;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_DISSOLVE: PopplerPageTransitionType = 5;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_GLITTER: PopplerPageTransitionType = 6;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_FLY: PopplerPageTransitionType = 7;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_PUSH: PopplerPageTransitionType = 8;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_COVER: PopplerPageTransitionType = 9;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_UNCOVER: PopplerPageTransitionType = 10;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_FADE: PopplerPageTransitionType = 11;
pub type PopplerPageTransitionType = u32;
pub const PopplerPageTransitionAlignment_POPPLER_PAGE_TRANSITION_HORIZONTAL:
    PopplerPageTransitionAlignment = 0;
pub const PopplerPageTransitionAlignment_POPPLER_PAGE_TRANSITION_VERTICAL:
    PopplerPageTransitionAlignment = 1;
pub type PopplerPageTransitionAlignment = u32;
pub const PopplerPageTransitionDirection_POPPLER_PAGE_TRANSITION_INWARD:
    PopplerPageTransitionDirection = 0;
pub const PopplerPageTransitionDirection_POPPLER_PAGE_TRANSITION_OUTWARD:
    PopplerPageTransitionDirection = 1;
pub type PopplerPageTransitionDirection = u32;
pub const PopplerSelectionStyle_POPPLER_SELECTION_GLYPH: PopplerSelectionStyle = 0;
pub const PopplerSelectionStyle_POPPLER_SELECTION_WORD: PopplerSelectionStyle = 1;
pub const PopplerSelectionStyle_POPPLER_SELECTION_LINE: PopplerSelectionStyle = 2;
pub type PopplerSelectionStyle = u32;
pub const PopplerPrintFlags_POPPLER_PRINT_DOCUMENT: PopplerPrintFlags = 0;
pub const PopplerPrintFlags_POPPLER_PRINT_MARKUP_ANNOTS: PopplerPrintFlags = 1;
pub const PopplerPrintFlags_POPPLER_PRINT_STAMP_ANNOTS_ONLY: PopplerPrintFlags = 2;
pub const PopplerPrintFlags_POPPLER_PRINT_ALL: PopplerPrintFlags = 1;
pub type PopplerPrintFlags = u32;
pub const PopplerFindFlags_POPPLER_FIND_DEFAULT: PopplerFindFlags = 0;
pub const PopplerFindFlags_POPPLER_FIND_CASE_SENSITIVE: PopplerFindFlags = 1;
pub const PopplerFindFlags_POPPLER_FIND_BACKWARDS: PopplerFindFlags = 2;
pub const PopplerFindFlags_POPPLER_FIND_WHOLE_WORDS_ONLY: PopplerFindFlags = 4;
pub const PopplerFindFlags_POPPLER_FIND_IGNORE_DIACRITICS: PopplerFindFlags = 8;
pub type PopplerFindFlags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerDocument {
    _unused: [u8; 0],
}
pub type PopplerDocument = _PopplerDocument;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerIndexIter {
    _unused: [u8; 0],
}
pub type PopplerIndexIter = _PopplerIndexIter;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerFontsIter {
    _unused: [u8; 0],
}
pub type PopplerFontsIter = _PopplerFontsIter;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerLayersIter {
    _unused: [u8; 0],
}
pub type PopplerLayersIter = _PopplerLayersIter;
pub type PopplerPoint = _PopplerPoint;
pub type PopplerRectangle = _PopplerRectangle;
pub type PopplerTextAttributes = _PopplerTextAttributes;
pub type PopplerColor = _PopplerColor;
pub type PopplerLinkMapping = _PopplerLinkMapping;
pub type PopplerPageTransition = _PopplerPageTransition;
pub type PopplerImageMapping = _PopplerImageMapping;
pub type PopplerFormFieldMapping = _PopplerFormFieldMapping;
pub type PopplerAnnotMapping = _PopplerAnnotMapping;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerPage {
    _unused: [u8; 0],
}
pub type PopplerPage = _PopplerPage;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerFontInfo {
    _unused: [u8; 0],
}
pub type PopplerFontInfo = _PopplerFontInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerLayer {
    _unused: [u8; 0],
}
pub type PopplerLayer = _PopplerLayer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerPSFile {
    _unused: [u8; 0],
}
pub type PopplerPSFile = _PopplerPSFile;
pub type PopplerAction = _PopplerAction;
pub type PopplerDest = _PopplerDest;
pub type PopplerActionLayer = _PopplerActionLayer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerFormField {
    _unused: [u8; 0],
}
pub type PopplerFormField = _PopplerFormField;
pub type PopplerAttachment = _PopplerAttachment;
pub type PopplerMovie = _PopplerMovie;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerMedia {
    _unused: [u8; 0],
}
pub type PopplerMedia = _PopplerMedia;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerAnnot {
    _unused: [u8; 0],
}
pub type PopplerAnnot = _PopplerAnnot;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerAnnotMarkup {
    _unused: [u8; 0],
}
pub type PopplerAnnotMarkup = _PopplerAnnotMarkup;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerAnnotText {
    _unused: [u8; 0],
}
pub type PopplerAnnotText = _PopplerAnnotText;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerAnnotTextMarkup {
    _unused: [u8; 0],
}
pub type PopplerAnnotTextMarkup = _PopplerAnnotTextMarkup;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerAnnotFreeText {
    _unused: [u8; 0],
}
pub type PopplerAnnotFreeText = _PopplerAnnotFreeText;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerAnnotFileAttachment {
    _unused: [u8; 0],
}
pub type PopplerAnnotFileAttachment = _PopplerAnnotFileAttachment;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerAnnotMovie {
    _unused: [u8; 0],
}
pub type PopplerAnnotMovie = _PopplerAnnotMovie;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerAnnotScreen {
    _unused: [u8; 0],
}
pub type PopplerAnnotScreen = _PopplerAnnotScreen;
pub type PopplerAnnotCalloutLine = _PopplerAnnotCalloutLine;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerAnnotLine {
    _unused: [u8; 0],
}
pub type PopplerAnnotLine = _PopplerAnnotLine;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerAnnotCircle {
    _unused: [u8; 0],
}
pub type PopplerAnnotCircle = _PopplerAnnotCircle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerAnnotSquare {
    _unused: [u8; 0],
}
pub type PopplerAnnotSquare = _PopplerAnnotSquare;
pub type PopplerQuadrilateral = _PopplerQuadrilateral;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerStructureElement {
    _unused: [u8; 0],
}
pub type PopplerStructureElement = _PopplerStructureElement;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerStructureElementIter {
    _unused: [u8; 0],
}
pub type PopplerStructureElementIter = _PopplerStructureElementIter;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerTextSpan {
    _unused: [u8; 0],
}
pub type PopplerTextSpan = _PopplerTextSpan;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerPageRange {
    _unused: [u8; 0],
}
pub type PopplerPageRange = _PopplerPageRange;
pub const PopplerBackend_POPPLER_BACKEND_UNKNOWN: PopplerBackend = 0;
pub const PopplerBackend_POPPLER_BACKEND_SPLASH: PopplerBackend = 1;
pub const PopplerBackend_POPPLER_BACKEND_CAIRO: PopplerBackend = 2;
pub type PopplerBackend = u32;
extern "C" {
    pub fn poppler_get_backend() -> PopplerBackend;
}
extern "C" {
    pub fn poppler_get_version() -> *const ::std::os::raw::c_char;
}
