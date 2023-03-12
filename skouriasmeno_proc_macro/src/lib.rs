use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Σφα" => "Err",
        "Οκ" => "Ok",
        "Συμβολοσειρα" => "String",
        "Λεξικο" => "HashMap",
        "Προκαθορισμενο" => "Default",
        "Σφαλμα" => "Error",
        "Επιλογη" => "Option",
        "Καποιο" => "Some",
        "Τιποτα" => "None",
        "Αποτελεσμα" => "Result",
        "Εαυτο" => "Self",
        "τυποσεγρμ" => "println",
        "σπασε" => "break",
        "ασυγχρονο" => "async",
        "περιμενε" => "await",
        "κυκλο" => "loop",
        "μετακινησε" => "move",
        "κυβωτιο" => "crate",
        "ανεφικτος_κωδικας" => "unreachable_code",
        "ως" => "as",
        "συνεχης" => "const",
        "χαρακτηριστικο" => "trait",
        "επικινδυνο" => "unsafe",
        "στο" => "in",
        "απο" => "from",
        "δυναμικο" => "dyn",
        "ξεδλιπλωσε" => "unwrap",
        "προκαθορισμενο" => "default",
        "ως_ανα" => "as_ref",
        "ιο" => "io",
        "εξωτερικο" => "extern",
        "ψευδης" => "false",
        "μεθοδος" => "fn",
        "σουπερ" => "super",
        "βαλε" => "insert",
        "παρε" => "get",
        "επιτρεψε" => "allow",
        "πανικος" => "panic",
        "μοναδα" => "mod",
        "μεταβλητος" => "mut",
        "νεο" => "new",
        "που" => "where",
        "για" => "for",
        "παρε_η_βαλε_με" => "get_or_insert_with",
        "κυριο" => "main",
        "δημοσιο" => "pub",
        "τιποτα" => None?,
        "επιστροφη" => "return",
        "εφαρμοσε" => "impl",
        "αναφορα" => "ref",
        "ταιριασε" => "match",
        "εαν" => "if",
        "τοτε" => "else",
        "εαυτος" => "self",
        "ασε" => "let",
        "στατικο" => "static",
        "κατασκευασμα" => "struct",
        "πιστευω" => "expect",
        "ενω" => "while",
        "χρησιμοποιησε" => "use",
        "σε" => "into",
        "αληθες" => "true",
        "απαριθμηση" => "enum",

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
pub fn skouriasmeno(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
