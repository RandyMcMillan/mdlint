use comrak::nodes::AstNode;
use crate::rules::common_checks::check_content;
use crate::rules::extensions::VecExt;
use crate::ruleset::RuleResult;

pub fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let details = check_content(root, r"^#+?[^\s]\S+", None);

    RuleResult::new(
        "MD018",
        "no-missing-space-atx",
        "No space after hash on atx style header",
        details.to_option(),
    )
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::parser::get_ast;
    use crate::rules::common_tests;
    use typed_arena::Arena;

    #[test]
    fn it_does_not_have_details_if_all_ok() {
        common_tests::all_ok("fixtures/md018/md018_ok.md", Box::new(check));
    }

    #[test]
    fn it_has_details_if_ko() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md018/md018_ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 2);
        let first = &details[0];
        assert_eq!(first.line, 1);
        assert_eq!(first.column, 1);
        assert_eq!(first.content, "#Everything is ok");
        let second = &details[1];
        assert_eq!(second.line, 5);
        assert_eq!(second.column, 1);
        assert_eq!(second.content, "##Lists & Codeblock");
    }
}
