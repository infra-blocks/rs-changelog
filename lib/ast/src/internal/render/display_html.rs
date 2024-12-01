use crate::block::LinkReferenceDefinition;

pub(crate) trait DisplayHtml {
    fn display_html(
        &self,
        buffer: &mut String,
        link_reference_definitions: &[LinkReferenceDefinition],
    );
}
