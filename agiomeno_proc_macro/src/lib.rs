use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "εχασκίαστειν" => "Err",
        "ούλα_καλά" => "Ok",
        "" => "String",
        "" => "HashMap",
        "Σασμένο" => "Default",
        "Χασκιαστήρη" => "Error",
        "" => "Option",
        "Κάτι_τις" => "Some",
        "Τίποτες" => "None",
        "ίγκομα" => "Result",
        "Εγιώνη" => "Self",
        "" => "println",
        "" => "print",
        "" => "format",
        "σπάσμα" => "break",
        "" => "async",
        "πόμεινε" => "await",
        "φάκκα_γυρό" => "loop",
        "τάραξε_το" => "move",
        "κάσια" => "crate",
        "" => "unreachable_code",
        "" => "as",
        "σταχερόν" => "const",
        "" => "trait",
        "φοϊτσιάρικο" => "unsafe",
        "μες_το" => "in",
        "που_το" => "from",
        "" => "dyn",
        "" => "unwrap",
        "σασμένο" => "default",
        "γυρίλλας" => "as_ref",
        "" => "io",
        "που_πώξω" => "extern",
        "λάχος" => "false",
        "" => "fn",
        "σούππερ" => "super",
        "μπήξε" => "insert",
        "πιάε" | "φερε" => "get",
        "άηκε" => "allow",
        "φάλλαρε" => "panic",
        "" => "mod",
        "" => "mut",
        "τζαινούρκο" => "new",
        "τζιαμέ_που" => "where",
        "" => "for",
        "πιάει_για_μπείξε_με" => "get_or_insert_with",
        "αντακώννω" => "main",
        "χαλίτικο" => "pub",
        "τίποτες" => None?,
        "ρέξε_δα" | "στράφου πισω" => "return",
        "" => "impl",
        "" => "ref",
        "τσιάττησε" | "αξάμωσε" => "match",
        "άμα" => "if",
        "ηδεμή" => "else",
        "εσούνη" => "self",
        "κάμε" => "let",
        "παλλουκομένο" => "static",
        "" => "struct",
        "σσιωωωω" | "ιστέ_ποϊλέ" => "expect",
        "ώσπου" | "ώστι" => "while",
        "" => "use",
        "μέσε" => "into",
        "" => "true",
        "" => "enum",
        "συνάξε" => "collect",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn agioma(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
