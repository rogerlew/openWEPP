use quick_xml::Reader;
use quick_xml::events::Event;

use crate::{AssuranceError, Result};

use super::{
    Figure, Report, ReportIds, RequiredNullable, require_nonempty, required_path,
    validate_reference_list,
};

const FORBIDDEN_ELEMENTS: &[&str] = &[
    "a",
    "animate",
    "animatemotion",
    "animatetransform",
    "discard",
    "foreignobject",
    "iframe",
    "image",
    "script",
    "set",
    "style",
    "metadata",
];
const MATPLOTLIB_DEFAULT_STYLE: &str =
    r#"<style type="text/css">*{stroke-linejoin: round; stroke-linecap: butt}</style>"#;

pub(super) fn validate_figure_contract(
    figure: &Figure,
    report: &Report,
    ids: &ReportIds,
) -> Result<()> {
    match figure.visualization.as_str() {
        "linear_magnitude_bars" => {
            if figure.kind != "result_bearing" {
                return Err(AssuranceError::Invalid(format!(
                    "generated figure '{}' must be result_bearing",
                    figure.id
                )));
            }
            if figure.result_ids.is_empty() || figure.value_binding_ids.is_empty() {
                return Err(AssuranceError::Invalid(format!(
                    "result-bearing figure '{}' requires result and value bindings",
                    figure.id
                )));
            }
            require_null_figure_object(
                &figure.research_object_id,
                &figure.id,
                "research_object_id",
            )?;
            require_null_figure_object(
                &figure.ancillary_object_id,
                &figure.id,
                "ancillary_object_id",
            )?;
            validate_reference_list(&figure.result_ids, &ids.results, "result", false)?;
            validate_reference_list(
                &figure.value_binding_ids,
                &ids.value_bindings,
                "value binding",
                true,
            )?;
        }
        "retained_svg" => {
            if figure.kind != "retained_evidence" {
                return Err(AssuranceError::Invalid(format!(
                    "retained SVG figure '{}' must be retained_evidence",
                    figure.id
                )));
            }
            if !figure.result_ids.is_empty() || !figure.value_binding_ids.is_empty() {
                return Err(AssuranceError::Invalid(format!(
                    "retained SVG figure '{}' cannot declare strict-result value bindings",
                    figure.id
                )));
            }
            let source_id = required_figure_object(
                &figure.research_object_id,
                &figure.id,
                "research_object_id",
            )?;
            let ancillary_id = required_figure_object(
                &figure.ancillary_object_id,
                &figure.id,
                "ancillary_object_id",
            )?;
            if source_id == ancillary_id {
                return Err(AssuranceError::Invalid(format!(
                    "retained SVG figure '{}' requires distinct source and ancillary objects",
                    figure.id
                )));
            }
            validate_reference_list(
                &[source_id.to_owned(), ancillary_id.to_owned()],
                &ids.research_objects,
                "research object",
                true,
            )?;
            validate_figure_object(report, source_id, "svg")?;
            validate_figure_object(report, ancillary_id, "md")?;
        }
        _ => {
            return Err(AssuranceError::Invalid(format!(
                "figure '{}' has unsupported visualization",
                figure.id
            )));
        }
    }
    require_nonempty(&figure.generation_procedure, "figure generation_procedure")?;
    require_nonempty(&figure.alternative_text, "figure alternative_text")?;
    require_nonempty(&figure.caption, "figure caption")
}

fn require_null_figure_object(
    value: &RequiredNullable<String>,
    figure_id: &str,
    field: &str,
) -> Result<()> {
    match value {
        RequiredNullable::Null | RequiredNullable::Missing => Ok(()),
        RequiredNullable::Value(_) => Err(AssuranceError::Invalid(format!(
            "generated figure '{figure_id}' cannot declare '{field}'"
        ))),
    }
}

pub(super) fn required_figure_object<'a>(
    value: &'a RequiredNullable<String>,
    figure_id: &str,
    field: &str,
) -> Result<&'a str> {
    match value {
        RequiredNullable::Value(value) => Ok(value),
        RequiredNullable::Null | RequiredNullable::Missing => Err(AssuranceError::Invalid(
            format!("retained SVG figure '{figure_id}' requires '{field}'"),
        )),
    }
}

fn validate_figure_object(report: &Report, object_id: &str, extension: &str) -> Result<()> {
    let object = report
        .research_objects
        .iter()
        .find(|object| object.id == object_id)
        .ok_or_else(|| {
            AssuranceError::Invalid(format!("unknown figure research object '{object_id}'"))
        })?;
    if object.access != "public_safe" {
        return Err(AssuranceError::Invalid(format!(
            "figure research object '{object_id}' must be public_safe"
        )));
    }
    let path = required_path(
        object.path.as_deref(),
        "retained figure research-object path",
    )?;
    if path.extension().and_then(|value| value.to_str()) != Some(extension) {
        return Err(AssuranceError::Invalid(format!(
            "figure research object '{object_id}' must have .{extension} extension"
        )));
    }
    Ok(())
}

// The event loop is intentionally kept together so XML nesting and accessibility
// state cannot diverge across helper boundaries.
#[allow(clippy::too_many_lines)]
pub(super) fn validate_retained_svg(bytes: &[u8]) -> Result<()> {
    let source = std::str::from_utf8(bytes)
        .map_err(|error| AssuranceError::Invalid(format!("retained SVG is not UTF-8: {error}")))?;
    reject_declarations(source)?;

    let mut reader = Reader::from_reader(bytes);
    reader.config_mut().trim_text(false);
    let mut root_seen = false;
    let mut root_closed = false;
    let mut depth = 0_u32;
    let mut in_title = false;
    let mut in_description = false;
    let mut title_text = String::new();
    let mut description_text = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(element)) => {
                if root_closed {
                    return Err(AssuranceError::Invalid(
                        "retained SVG has content after the root element".to_owned(),
                    ));
                }
                let name = element_name(element.name().as_ref())?;
                if root_seen {
                    validate_element(&element, &name)?;
                } else {
                    validate_root(&element, &name)?;
                    root_seen = true;
                }
                depth = depth.checked_add(1).ok_or_else(|| {
                    AssuranceError::Invalid("retained SVG nesting overflowed".to_owned())
                })?;
                in_title = name == "title";
                in_description = name == "desc";
            }
            Ok(Event::Empty(element)) => {
                if root_closed {
                    return Err(AssuranceError::Invalid(
                        "retained SVG has content after the root element".to_owned(),
                    ));
                }
                let name = element_name(element.name().as_ref())?;
                if !root_seen {
                    return Err(AssuranceError::Invalid(
                        "retained SVG root cannot be empty".to_owned(),
                    ));
                }
                validate_element(&element, &name)?;
            }
            Ok(Event::Text(text)) => {
                let text = text.decode().map_err(|error| {
                    AssuranceError::Invalid(format!("retained SVG text is invalid: {error}"))
                })?;
                if depth == 0 && !text.trim().is_empty() {
                    return Err(AssuranceError::Invalid(
                        "retained SVG has text outside the root element".to_owned(),
                    ));
                }
                if in_title {
                    title_text.push_str(&text);
                }
                if in_description {
                    description_text.push_str(&text);
                }
            }
            Ok(Event::CData(text)) => {
                if depth == 0 {
                    return Err(AssuranceError::Invalid(
                        "retained SVG has CDATA outside the root element".to_owned(),
                    ));
                }
                let text = text.decode().map_err(|error| {
                    AssuranceError::Invalid(format!("retained SVG CDATA is invalid: {error}"))
                })?;
                if in_title {
                    title_text.push_str(&text);
                }
                if in_description {
                    description_text.push_str(&text);
                }
            }
            Ok(Event::End(element)) => {
                let name = element_name(element.name().as_ref())?;
                if depth == 0 {
                    return Err(AssuranceError::Invalid(
                        "retained SVG has an unmatched closing element".to_owned(),
                    ));
                }
                depth -= 1;
                if name == "title" {
                    in_title = false;
                }
                if name == "desc" {
                    in_description = false;
                }
                if depth == 0 {
                    if name != "svg" {
                        return Err(AssuranceError::Invalid(
                            "retained SVG root must be svg".to_owned(),
                        ));
                    }
                    root_closed = true;
                }
            }
            Ok(Event::Comment(_)) => {}
            Ok(Event::Eof) => break,
            Ok(Event::Decl(_) | Event::DocType(_) | Event::PI(_) | Event::GeneralRef(_)) => {
                return Err(AssuranceError::Invalid(
                    "retained SVG contains a prohibited XML construct".to_owned(),
                ));
            }
            Err(error) => {
                return Err(AssuranceError::Invalid(format!(
                    "retained SVG XML parse failed: {error}"
                )));
            }
        }
    }

    if !root_seen || !root_closed || depth != 0 {
        return Err(AssuranceError::Invalid(
            "retained SVG does not contain one complete root element".to_owned(),
        ));
    }
    if title_text.trim().is_empty() || description_text.trim().is_empty() {
        return Err(AssuranceError::Invalid(
            "retained SVG requires nonempty title and desc text".to_owned(),
        ));
    }
    Ok(())
}

pub(super) fn sanitize_retained_svg(
    bytes: &[u8],
    title: &str,
    description: &str,
) -> Result<Vec<u8>> {
    let source = std::str::from_utf8(bytes)
        .map_err(|error| AssuranceError::Invalid(format!("retained SVG is not UTF-8: {error}")))?;
    let mut remaining = source.trim_start_matches(char::is_whitespace);
    if remaining.starts_with("<?xml") {
        let end = remaining.find("?>").ok_or_else(|| {
            AssuranceError::Invalid("retained SVG has an unterminated XML declaration".to_owned())
        })?;
        remaining = remaining[end + 2..].trim_start_matches(char::is_whitespace);
    }
    if remaining
        .get(..9)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("<!doctype"))
    {
        let end = remaining.find('>').ok_or_else(|| {
            AssuranceError::Invalid("retained SVG has an unterminated DOCTYPE".to_owned())
        })?;
        let declaration = &remaining[..=end];
        if declaration.contains('[') {
            return Err(AssuranceError::Invalid(
                "retained SVG DOCTYPE cannot contain an internal subset".to_owned(),
            ));
        }
        remaining = remaining[end + 1..].trim_start_matches(char::is_whitespace);
    }
    let mut sanitized = remaining.to_owned();
    if let Some(metadata_start) = sanitized.find("<metadata>") {
        let relative_end = sanitized[metadata_start..]
            .find("</metadata>")
            .ok_or_else(|| {
                AssuranceError::Invalid(
                    "retained SVG has an unterminated metadata element".to_owned(),
                )
            })?;
        let metadata_end = metadata_start + relative_end + "</metadata>".len();
        sanitized.replace_range(metadata_start..metadata_end, "");
    }
    let matplotlib_style = sanitized.contains(MATPLOTLIB_DEFAULT_STYLE);
    if matplotlib_style {
        sanitized = sanitized.replace(MATPLOTLIB_DEFAULT_STYLE, "");
    }
    let root_start = sanitized.find("<svg").ok_or_else(|| {
        AssuranceError::Invalid("retained SVG does not contain an svg root".to_owned())
    })?;
    let root_end = sanitized[root_start..]
        .find('>')
        .ok_or_else(|| AssuranceError::Invalid("retained SVG root is unterminated".to_owned()))?
        + root_start;
    if sanitized[root_start..root_end].contains(" role=") {
        return Err(AssuranceError::Invalid(
            "retained SVG source must not supply its own role".to_owned(),
        ));
    }
    let safe_root_attributes = if matplotlib_style {
        r#" role="img" stroke-linejoin="round" stroke-linecap="butt""#
    } else {
        r#" role="img""#
    };
    sanitized.insert_str(root_end, safe_root_attributes);
    let injection_point = root_end + safe_root_attributes.len() + 1;
    sanitized.insert_str(
        injection_point,
        &format!(
            "\n <title>{}</title>\n <desc>{}</desc>",
            xml_escape(title),
            xml_escape(description)
        ),
    );
    let mut sanitized = sanitized.into_bytes();
    if !sanitized.ends_with(b"\n") {
        sanitized.push(b'\n');
    }
    validate_retained_svg(&sanitized)?;
    Ok(sanitized)
}

fn reject_declarations(source: &str) -> Result<()> {
    let lowercase = source.to_ascii_lowercase();
    if lowercase.contains("<!doctype")
        || lowercase.contains("<!entity")
        || lowercase.contains("<?xml-stylesheet")
    {
        return Err(AssuranceError::Invalid(
            "retained SVG contains a prohibited declaration".to_owned(),
        ));
    }
    Ok(())
}

fn validate_root(element: &quick_xml::events::BytesStart<'_>, name: &str) -> Result<()> {
    if name != "svg" {
        return Err(AssuranceError::Invalid(
            "retained SVG root must be svg".to_owned(),
        ));
    }
    validate_attributes(element, true)
}

fn validate_element(element: &quick_xml::events::BytesStart<'_>, name: &str) -> Result<()> {
    if FORBIDDEN_ELEMENTS.contains(&name) {
        return Err(AssuranceError::Invalid(format!(
            "retained SVG contains prohibited element '{name}'"
        )));
    }
    validate_attributes(element, false)
}

fn validate_attributes(element: &quick_xml::events::BytesStart<'_>, root: bool) -> Result<()> {
    let mut namespace_ok = false;
    let mut role_ok = false;
    for attribute in element.attributes() {
        let attribute = attribute.map_err(|error| {
            AssuranceError::Invalid(format!("retained SVG attribute is invalid: {error}"))
        })?;
        let name = std::str::from_utf8(attribute.key.as_ref())
            .map_err(|_| AssuranceError::Invalid("retained SVG attribute is not UTF-8".to_owned()))?
            .to_ascii_lowercase();
        let value = attribute
            .decoded_and_normalized_value(quick_xml::XmlVersion::Implicit1_0, element.decoder())
            .map_err(|error| {
                AssuranceError::Invalid(format!("retained SVG attribute value is invalid: {error}"))
            })?;
        let lowercase = value.to_ascii_lowercase();
        if name.contains(':')
            && !matches!(name.as_str(), "xlink:href" | "xmlns:xlink" | "xml:space")
        {
            return Err(AssuranceError::Invalid(format!(
                "retained SVG contains unsupported namespaced attribute '{name}'"
            )));
        }
        if value.contains('\\') {
            return Err(AssuranceError::Invalid(format!(
                "retained SVG attribute '{name}' contains a prohibited CSS escape"
            )));
        }
        if name.starts_with("on")
            || matches!(name.as_str(), "src" | "data" | "formaction" | "xml:base")
        {
            return Err(AssuranceError::Invalid(format!(
                "retained SVG contains prohibited attribute '{name}'"
            )));
        }
        if matches!(name.as_str(), "href" | "xlink:href") && !lowercase.starts_with('#') {
            return Err(AssuranceError::Invalid(format!(
                "retained SVG attribute '{name}' must be an internal fragment"
            )));
        }
        if name == "style" {
            validate_safe_style(&lowercase)?;
        }
        if lowercase.contains("javascript:")
            || lowercase.contains("data:")
            || lowercase.contains("@import")
            || (lowercase.contains("://")
                && !(name.starts_with("xmlns")
                    && (lowercase == "http://www.w3.org/2000/svg"
                        || lowercase == "http://www.w3.org/1999/xlink")))
            || contains_external_url(&lowercase)
        {
            return Err(AssuranceError::Invalid(format!(
                "retained SVG attribute '{name}' can load external or active content"
            )));
        }
        if root && name == "xmlns" && lowercase == "http://www.w3.org/2000/svg" {
            namespace_ok = true;
        }
        if root && name == "role" && lowercase == "img" {
            role_ok = true;
        }
    }
    if root && (!namespace_ok || !role_ok) {
        return Err(AssuranceError::Invalid(
            "retained SVG root requires the SVG namespace and role=img".to_owned(),
        ));
    }
    Ok(())
}

fn validate_safe_style(style: &str) -> Result<()> {
    for declaration in style.split(';').filter(|value| !value.trim().is_empty()) {
        let (property, value) = declaration.split_once(':').ok_or_else(|| {
            AssuranceError::Invalid("retained SVG style declaration is malformed".to_owned())
        })?;
        let property = property.trim();
        let value = value.trim();
        let valid = match property {
            "fill" | "stroke" => value == "none" || valid_hex_color(value),
            "fill-opacity" | "stroke-opacity" => valid_unit_interval(value),
            "stroke-width" | "stroke-dashoffset" => valid_nonnegative_number(value),
            "stroke-dasharray" => value
                .split(',')
                .all(|component| valid_nonnegative_number(component.trim())),
            "stroke-linecap" => matches!(value, "butt" | "round" | "square"),
            "stroke-linejoin" => matches!(value, "miter" | "round" | "bevel"),
            "font-family" => value == "'dejavu sans'",
            "font-size" => value.strip_suffix("px").is_some_and(valid_positive_number),
            "text-anchor" => matches!(value, "start" | "middle" | "end"),
            _ => false,
        };
        if !valid {
            return Err(AssuranceError::Invalid(format!(
                "retained SVG style property '{property}' has an unsupported value"
            )));
        }
    }
    Ok(())
}

fn valid_hex_color(value: &str) -> bool {
    value
        .strip_prefix('#')
        .is_some_and(|hex| hex.len() == 6 && hex.bytes().all(|byte| byte.is_ascii_hexdigit()))
}

fn valid_unit_interval(value: &str) -> bool {
    value
        .parse::<f64>()
        .is_ok_and(|number| number.is_finite() && (0.0..=1.0).contains(&number))
}

fn valid_nonnegative_number(value: &str) -> bool {
    value
        .parse::<f64>()
        .is_ok_and(|number| number.is_finite() && number >= 0.0)
}

fn valid_positive_number(value: &str) -> bool {
    value
        .parse::<f64>()
        .is_ok_and(|number| number.is_finite() && number > 0.0)
}

fn contains_external_url(value: &str) -> bool {
    let mut remaining = value;
    while let Some(start) = remaining.find("url(") {
        let after = &remaining[start + 4..];
        let Some(end) = after.find(')') else {
            return true;
        };
        let target = after[..end].trim().trim_matches(['\'', '"']);
        if !target.starts_with('#') {
            return true;
        }
        remaining = &after[end + 1..];
    }
    false
}

fn element_name(name: &[u8]) -> Result<String> {
    let name = std::str::from_utf8(name)
        .map_err(|_| AssuranceError::Invalid("retained SVG element is not UTF-8".to_owned()))?;
    if name.contains(':') {
        return Err(AssuranceError::Invalid(format!(
            "retained SVG contains unsupported prefixed element '{name}'"
        )));
    }
    Ok(name.to_ascii_lowercase())
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::{MATPLOTLIB_DEFAULT_STYLE, sanitize_retained_svg, validate_retained_svg};

    const VALID: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 10 10"><title>Trend</title><desc>Accessible trend description</desc><defs><pattern id="p"/></defs><rect width="10" height="10" fill="url(#p)"/></svg>"#;

    #[test]
    fn accepts_accessible_inert_svg() {
        validate_retained_svg(VALID.as_bytes()).expect("valid retained SVG");
    }

    #[test]
    fn rejects_active_and_external_content() {
        for invalid in [
            VALID.replace("<rect", "<script/><rect"),
            VALID.replace("<rect", "<image href=\"https://example.test/x\"/><rect"),
            VALID.replace(
                "<rect",
                "<style>@import url(https://example.test/x.css)</style><rect",
            ),
            VALID.replace(
                "<rect",
                "<rect style=\"fill: \\\\75rl(https://example.test/x)\"",
            ),
            VALID.replace(
                "<rect",
                "<rect fill=\"\\\\75rl(\\\\68ttps\\\\3a//example.test/x)\"",
            ),
            VALID.replace(
                "<rect",
                "<g xml:base=\"//example.test/x\"><use href=\"#p\"/></g><rect",
            ),
            VALID.replace(
                "<rect",
                "<use xmlns:e=\"http://www.w3.org/1999/xlink\" e:href=\"//example.test/x\"/><rect",
            ),
            VALID.replace("<rect", "<e:g/><rect"),
            VALID.replace("<defs>", "<metadata id=\"x\">unsafe</metadata><defs>"),
            VALID.replace("<rect", "<rect onclick=\"run()\""),
            VALID.replace("url(#p)", "url(https://example.test/p)"),
            VALID.replace("<title>Trend</title>", "<title></title>"),
            VALID.replace("role=\"img\"", "role=\"presentation\""),
            format!("<!DOCTYPE svg>{VALID}"),
            format!("{VALID}<rect/>"),
            format!("garbage{VALID}"),
            format!("{VALID}garbage"),
            format!("{VALID}<![CDATA[garbage]]>"),
            format!("<?xml version=\"1.0\"?>{VALID}"),
        ] {
            assert!(
                validate_retained_svg(invalid.as_bytes()).is_err(),
                "must reject {invalid}"
            );
        }
    }

    #[test]
    fn strips_only_leading_xml_and_external_doctype_declarations() {
        let source_svg = VALID.replace(" role=\"img\"", "").replace(
            "<title>Trend</title><desc>Accessible trend description</desc>",
            "",
        );
        let source = format!(
            "<?xml version=\"1.0\"?>\n<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n{source_svg}"
        );
        let sanitized =
            sanitize_retained_svg(source.as_bytes(), "Trend", "Accessible trend description")
                .expect("sanitize");
        validate_retained_svg(&sanitized).expect("sanitized SVG");
        assert!(
            sanitize_retained_svg(
                format!("<!DOCTYPE svg [<!ENTITY x \"y\">]>{source_svg}").as_bytes(),
                "Trend",
                "Description",
            )
            .is_err()
        );
    }

    #[test]
    fn converts_only_the_inert_matplotlib_default_style() {
        let source = VALID
            .replace(" role=\"img\"", "")
            .replace("<defs>", &format!("{MATPLOTLIB_DEFAULT_STYLE}<defs>"));
        let sanitized =
            sanitize_retained_svg(source.as_bytes(), "Trend", "Accessible trend description")
                .expect("sanitize Matplotlib SVG");
        let sanitized = String::from_utf8(sanitized).expect("UTF-8");
        assert!(!sanitized.contains("<style"));
        assert!(sanitized.contains(r#"stroke-linejoin="round" stroke-linecap="butt""#));
        validate_retained_svg(sanitized.as_bytes()).expect("valid sanitized SVG");

        let active = source.replace(
            MATPLOTLIB_DEFAULT_STYLE,
            "<style>@import url(https://example.test/x.css)</style>",
        );
        assert!(
            sanitize_retained_svg(active.as_bytes(), "Trend", "Description").is_err(),
            "must not normalize arbitrary CSS"
        );
    }
}
