/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerPageRange {
    _unused: [u8; 0],
}
pub type PopplerPageRange = _PopplerPageRange;
extern "C" {
    pub fn poppler_page_get_type() -> GType;
}
extern "C" {
    pub fn poppler_page_render(page: *mut PopplerPage, cairo: *mut cairo_t);
}
extern "C" {
    pub fn poppler_page_render_for_printing(page: *mut PopplerPage, cairo: *mut cairo_t);
}
extern "C" {
    pub fn poppler_page_render_for_printing_with_options(
        page: *mut PopplerPage,
        cairo: *mut cairo_t,
        options: PopplerPrintFlags,
    );
}
extern "C" {
    pub fn poppler_page_get_thumbnail(page: *mut PopplerPage) -> *mut cairo_surface_t;
}
extern "C" {
    pub fn poppler_page_render_selection(
        page: *mut PopplerPage,
        cairo: *mut cairo_t,
        selection: *mut PopplerRectangle,
        old_selection: *mut PopplerRectangle,
        style: PopplerSelectionStyle,
        glyph_color: *mut PopplerColor,
        background_color: *mut PopplerColor,
    );
}
extern "C" {
    pub fn poppler_page_get_size(page: *mut PopplerPage, width: *mut f64, height: *mut f64);
}
extern "C" {
    pub fn poppler_page_get_index(page: *mut PopplerPage) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn poppler_page_get_label(page: *mut PopplerPage) -> *mut gchar;
}
extern "C" {
    pub fn poppler_page_get_duration(page: *mut PopplerPage) -> f64;
}
extern "C" {
    pub fn poppler_page_get_transition(page: *mut PopplerPage) -> *mut PopplerPageTransition;
}
extern "C" {
    pub fn poppler_page_get_thumbnail_size(
        page: *mut PopplerPage,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> gboolean;
}
extern "C" {
    pub fn poppler_page_find_text_with_options(
        page: *mut PopplerPage,
        text: *const ::std::os::raw::c_char,
        options: PopplerFindFlags,
    ) -> *mut GList;
}
extern "C" {
    pub fn poppler_page_find_text(
        page: *mut PopplerPage,
        text: *const ::std::os::raw::c_char,
    ) -> *mut GList;
}
extern "C" {
    pub fn poppler_page_render_to_ps(page: *mut PopplerPage, ps_file: *mut PopplerPSFile);
}
extern "C" {
    pub fn poppler_page_get_text(page: *mut PopplerPage) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn poppler_page_get_text_for_area(
        page: *mut PopplerPage,
        area: *mut PopplerRectangle,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn poppler_page_get_selected_text(
        page: *mut PopplerPage,
        style: PopplerSelectionStyle,
        selection: *mut PopplerRectangle,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn poppler_page_get_selected_region(
        page: *mut PopplerPage,
        scale: gdouble,
        style: PopplerSelectionStyle,
        selection: *mut PopplerRectangle,
    ) -> *mut cairo_region_t;
}
extern "C" {
    pub fn poppler_page_get_selection_region(
        page: *mut PopplerPage,
        scale: gdouble,
        style: PopplerSelectionStyle,
        selection: *mut PopplerRectangle,
    ) -> *mut GList;
}
extern "C" {
    pub fn poppler_page_selection_region_free(region: *mut GList);
}
extern "C" {
    pub fn poppler_page_get_link_mapping(page: *mut PopplerPage) -> *mut GList;
}
extern "C" {
    pub fn poppler_page_free_link_mapping(list: *mut GList);
}
extern "C" {
    pub fn poppler_page_get_image_mapping(page: *mut PopplerPage) -> *mut GList;
}
extern "C" {
    pub fn poppler_page_free_image_mapping(list: *mut GList);
}
extern "C" {
    pub fn poppler_page_get_image(page: *mut PopplerPage, image_id: gint) -> *mut cairo_surface_t;
}
extern "C" {
    pub fn poppler_page_get_form_field_mapping(page: *mut PopplerPage) -> *mut GList;
}
extern "C" {
    pub fn poppler_page_free_form_field_mapping(list: *mut GList);
}
extern "C" {
    pub fn poppler_page_get_annot_mapping(page: *mut PopplerPage) -> *mut GList;
}
extern "C" {
    pub fn poppler_page_free_annot_mapping(list: *mut GList);
}
extern "C" {
    pub fn poppler_page_add_annot(page: *mut PopplerPage, annot: *mut PopplerAnnot);
}
extern "C" {
    pub fn poppler_page_remove_annot(page: *mut PopplerPage, annot: *mut PopplerAnnot);
}
extern "C" {
    pub fn poppler_page_get_crop_box(page: *mut PopplerPage, rect: *mut PopplerRectangle);
}
extern "C" {
    pub fn poppler_page_get_text_layout(
        page: *mut PopplerPage,
        rectangles: *mut *mut PopplerRectangle,
        n_rectangles: *mut guint,
    ) -> gboolean;
}
extern "C" {
    pub fn poppler_page_get_text_layout_for_area(
        page: *mut PopplerPage,
        area: *mut PopplerRectangle,
        rectangles: *mut *mut PopplerRectangle,
        n_rectangles: *mut guint,
    ) -> gboolean;
}
extern "C" {
    pub fn poppler_page_get_text_attributes(page: *mut PopplerPage) -> *mut GList;
}
extern "C" {
    pub fn poppler_page_free_text_attributes(list: *mut GList);
}
extern "C" {
    pub fn poppler_page_get_text_attributes_for_area(
        page: *mut PopplerPage,
        area: *mut PopplerRectangle,
    ) -> *mut GList;
}
#[repr(C)]
pub struct _PopplerRectangle {
    pub x1: gdouble,
    pub y1: gdouble,
    pub x2: gdouble,
    pub y2: gdouble,
}
#[test]
fn bindgen_test_layout__PopplerRectangle() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerRectangle>(),
        32usize,
        concat!("Size of: ", stringify!(_PopplerRectangle))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerRectangle>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerRectangle))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerRectangle>())).x1 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerRectangle),
            "::",
            stringify!(x1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerRectangle>())).y1 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerRectangle),
            "::",
            stringify!(y1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerRectangle>())).x2 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerRectangle),
            "::",
            stringify!(x2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerRectangle>())).y2 as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerRectangle),
            "::",
            stringify!(y2)
        )
    );
}
extern "C" {
    pub fn poppler_rectangle_get_type() -> GType;
}
extern "C" {
    pub fn poppler_rectangle_new() -> *mut PopplerRectangle;
}
extern "C" {
    pub fn poppler_rectangle_copy(rectangle: *mut PopplerRectangle) -> *mut PopplerRectangle;
}
extern "C" {
    pub fn poppler_rectangle_free(rectangle: *mut PopplerRectangle);
}
#[repr(C)]
pub struct _PopplerPoint {
    pub x: gdouble,
    pub y: gdouble,
}
#[test]
fn bindgen_test_layout__PopplerPoint() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerPoint>(),
        16usize,
        concat!("Size of: ", stringify!(_PopplerPoint))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerPoint>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerPoint))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerPoint>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerPoint),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerPoint>())).y as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerPoint),
            "::",
            stringify!(y)
        )
    );
}
extern "C" {
    pub fn poppler_point_get_type() -> GType;
}
extern "C" {
    pub fn poppler_point_new() -> *mut PopplerPoint;
}
extern "C" {
    pub fn poppler_point_copy(point: *mut PopplerPoint) -> *mut PopplerPoint;
}
extern "C" {
    pub fn poppler_point_free(point: *mut PopplerPoint);
}
#[repr(C)]
pub struct _PopplerQuadrilateral {
    pub p1: PopplerPoint,
    pub p2: PopplerPoint,
    pub p3: PopplerPoint,
    pub p4: PopplerPoint,
}
#[test]
fn bindgen_test_layout__PopplerQuadrilateral() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerQuadrilateral>(),
        64usize,
        concat!("Size of: ", stringify!(_PopplerQuadrilateral))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerQuadrilateral>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerQuadrilateral))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerQuadrilateral>())).p1 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerQuadrilateral),
            "::",
            stringify!(p1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerQuadrilateral>())).p2 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerQuadrilateral),
            "::",
            stringify!(p2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerQuadrilateral>())).p3 as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerQuadrilateral),
            "::",
            stringify!(p3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerQuadrilateral>())).p4 as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerQuadrilateral),
            "::",
            stringify!(p4)
        )
    );
}
extern "C" {
    pub fn poppler_quadrilateral_get_type() -> GType;
}
extern "C" {
    pub fn poppler_quadrilateral_new() -> *mut PopplerQuadrilateral;
}
extern "C" {
    pub fn poppler_quadrilateral_copy(quad: *mut PopplerQuadrilateral)
        -> *mut PopplerQuadrilateral;
}
extern "C" {
    pub fn poppler_quadrilateral_free(quad: *mut PopplerQuadrilateral);
}
#[repr(C)]
pub struct _PopplerColor {
    pub red: guint16,
    pub green: guint16,
    pub blue: guint16,
}
#[test]
fn bindgen_test_layout__PopplerColor() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerColor>(),
        6usize,
        concat!("Size of: ", stringify!(_PopplerColor))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerColor>(),
        2usize,
        concat!("Alignment of ", stringify!(_PopplerColor))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerColor>())).red as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerColor),
            "::",
            stringify!(red)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerColor>())).green as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerColor),
            "::",
            stringify!(green)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerColor>())).blue as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerColor),
            "::",
            stringify!(blue)
        )
    );
}
extern "C" {
    pub fn poppler_color_get_type() -> GType;
}
extern "C" {
    pub fn poppler_color_new() -> *mut PopplerColor;
}
extern "C" {
    pub fn poppler_color_copy(color: *mut PopplerColor) -> *mut PopplerColor;
}
extern "C" {
    pub fn poppler_color_free(color: *mut PopplerColor);
}
#[repr(C)]
pub struct _PopplerTextAttributes {
    pub font_name: *mut gchar,
    pub font_size: gdouble,
    pub is_underlined: gboolean,
    pub color: PopplerColor,
    pub start_index: gint,
    pub end_index: gint,
}
#[test]
fn bindgen_test_layout__PopplerTextAttributes() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerTextAttributes>(),
        40usize,
        concat!("Size of: ", stringify!(_PopplerTextAttributes))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerTextAttributes>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerTextAttributes))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerTextAttributes>())).font_name as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerTextAttributes),
            "::",
            stringify!(font_name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerTextAttributes>())).font_size as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerTextAttributes),
            "::",
            stringify!(font_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerTextAttributes>())).is_underlined as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerTextAttributes),
            "::",
            stringify!(is_underlined)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerTextAttributes>())).color as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerTextAttributes),
            "::",
            stringify!(color)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerTextAttributes>())).start_index as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerTextAttributes),
            "::",
            stringify!(start_index)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerTextAttributes>())).end_index as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerTextAttributes),
            "::",
            stringify!(end_index)
        )
    );
}
extern "C" {
    pub fn poppler_text_attributes_get_type() -> GType;
}
extern "C" {
    pub fn poppler_text_attributes_new() -> *mut PopplerTextAttributes;
}
extern "C" {
    pub fn poppler_text_attributes_copy(
        text_attrs: *mut PopplerTextAttributes,
    ) -> *mut PopplerTextAttributes;
}
extern "C" {
    pub fn poppler_text_attributes_free(text_attrs: *mut PopplerTextAttributes);
}
#[repr(C)]
pub struct _PopplerLinkMapping {
    pub area: PopplerRectangle,
    pub action: *mut PopplerAction,
}
#[test]
fn bindgen_test_layout__PopplerLinkMapping() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerLinkMapping>(),
        40usize,
        concat!("Size of: ", stringify!(_PopplerLinkMapping))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerLinkMapping>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerLinkMapping))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerLinkMapping>())).area as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerLinkMapping),
            "::",
            stringify!(area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerLinkMapping>())).action as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerLinkMapping),
            "::",
            stringify!(action)
        )
    );
}
extern "C" {
    pub fn poppler_link_mapping_get_type() -> GType;
}
extern "C" {
    pub fn poppler_link_mapping_new() -> *mut PopplerLinkMapping;
}
extern "C" {
    pub fn poppler_link_mapping_copy(mapping: *mut PopplerLinkMapping) -> *mut PopplerLinkMapping;
}
extern "C" {
    pub fn poppler_link_mapping_free(mapping: *mut PopplerLinkMapping);
}
#[repr(C)]
pub struct _PopplerPageTransition {
    pub type_: PopplerPageTransitionType,
    pub alignment: PopplerPageTransitionAlignment,
    pub direction: PopplerPageTransitionDirection,
    pub duration: gint,
    pub angle: gint,
    pub scale: gdouble,
    pub rectangular: gboolean,
    pub duration_real: gdouble,
}
#[test]
fn bindgen_test_layout__PopplerPageTransition() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerPageTransition>(),
        48usize,
        concat!("Size of: ", stringify!(_PopplerPageTransition))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerPageTransition>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerPageTransition))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerPageTransition>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerPageTransition),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerPageTransition>())).alignment as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerPageTransition),
            "::",
            stringify!(alignment)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerPageTransition>())).direction as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerPageTransition),
            "::",
            stringify!(direction)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerPageTransition>())).duration as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerPageTransition),
            "::",
            stringify!(duration)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerPageTransition>())).angle as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerPageTransition),
            "::",
            stringify!(angle)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerPageTransition>())).scale as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerPageTransition),
            "::",
            stringify!(scale)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerPageTransition>())).rectangular as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerPageTransition),
            "::",
            stringify!(rectangular)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerPageTransition>())).duration_real as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerPageTransition),
            "::",
            stringify!(duration_real)
        )
    );
}
extern "C" {
    pub fn poppler_page_transition_get_type() -> GType;
}
extern "C" {
    pub fn poppler_page_transition_new() -> *mut PopplerPageTransition;
}
extern "C" {
    pub fn poppler_page_transition_copy(
        transition: *mut PopplerPageTransition,
    ) -> *mut PopplerPageTransition;
}
extern "C" {
    pub fn poppler_page_transition_free(transition: *mut PopplerPageTransition);
}
#[repr(C)]
pub struct _PopplerImageMapping {
    pub area: PopplerRectangle,
    pub image_id: gint,
}
#[test]
fn bindgen_test_layout__PopplerImageMapping() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerImageMapping>(),
        40usize,
        concat!("Size of: ", stringify!(_PopplerImageMapping))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerImageMapping>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerImageMapping))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerImageMapping>())).area as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerImageMapping),
            "::",
            stringify!(area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerImageMapping>())).image_id as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerImageMapping),
            "::",
            stringify!(image_id)
        )
    );
}
extern "C" {
    pub fn poppler_image_mapping_get_type() -> GType;
}
extern "C" {
    pub fn poppler_image_mapping_new() -> *mut PopplerImageMapping;
}
extern "C" {
    pub fn poppler_image_mapping_copy(
        mapping: *mut PopplerImageMapping,
    ) -> *mut PopplerImageMapping;
}
extern "C" {
    pub fn poppler_image_mapping_free(mapping: *mut PopplerImageMapping);
}
#[repr(C)]
pub struct _PopplerFormFieldMapping {
    pub area: PopplerRectangle,
    pub field: *mut PopplerFormField,
}
#[test]
fn bindgen_test_layout__PopplerFormFieldMapping() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerFormFieldMapping>(),
        40usize,
        concat!("Size of: ", stringify!(_PopplerFormFieldMapping))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerFormFieldMapping>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerFormFieldMapping))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerFormFieldMapping>())).area as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerFormFieldMapping),
            "::",
            stringify!(area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerFormFieldMapping>())).field as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerFormFieldMapping),
            "::",
            stringify!(field)
        )
    );
}
extern "C" {
    pub fn poppler_form_field_mapping_get_type() -> GType;
}
extern "C" {
    pub fn poppler_form_field_mapping_new() -> *mut PopplerFormFieldMapping;
}
extern "C" {
    pub fn poppler_form_field_mapping_copy(
        mapping: *mut PopplerFormFieldMapping,
    ) -> *mut PopplerFormFieldMapping;
}
extern "C" {
    pub fn poppler_form_field_mapping_free(mapping: *mut PopplerFormFieldMapping);
}
#[repr(C)]
pub struct _PopplerAnnotMapping {
    pub area: PopplerRectangle,
    pub annot: *mut PopplerAnnot,
}
#[test]
fn bindgen_test_layout__PopplerAnnotMapping() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerAnnotMapping>(),
        40usize,
        concat!("Size of: ", stringify!(_PopplerAnnotMapping))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerAnnotMapping>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerAnnotMapping))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotMapping>())).area as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotMapping),
            "::",
            stringify!(area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotMapping>())).annot as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotMapping),
            "::",
            stringify!(annot)
        )
    );
}
extern "C" {
    pub fn poppler_annot_mapping_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_mapping_new() -> *mut PopplerAnnotMapping;
}
extern "C" {
    pub fn poppler_annot_mapping_copy(
        mapping: *mut PopplerAnnotMapping,
    ) -> *mut PopplerAnnotMapping;
}
extern "C" {
    pub fn poppler_annot_mapping_free(mapping: *mut PopplerAnnotMapping);
}
