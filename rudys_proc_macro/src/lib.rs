use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Klaid" => "Err",
        "Gerai" => "Ok",
        "Eilutė" => "String",
        "Hašmapas" => "HashMap",
        "Numatytas" => "Default",
        "Klaida" => "Error",
        "Parinktis" => "Option",
        "Kažkoks" => "Some",
        "Joks" => "None",
        "Rezultatas" => "Result",
        "Pats" => "Self",
        "spausdintliniją" => "println",
        "pertraukt" => "break",
        "asinchroninis" => "async",
        "laukt" => "await",
        "ciklas" => "loop",
        "perkelt" => "move",
        "dėžė" => "crate",
        "nepasiekiamas_kodas" => "unreachable_code",
        "kaip" => "as",
        "pastovus" => "const",
        "bruožas" => "trait",
        "nesaugus" => "unsafe",
        "jame" => "in",
        "iš" => "from",
        "dinamiškas" => "dyn",
        "išpakuot" => "unwrap",
        "numatytas" => "default",
        "kaip_nuoroda" => "as_ref",
        "įvesties_išvesties" => "io",
        "išorinis" => "extern",
        "melas" => "false",
        "funkcija" => "fn",
        "puikus" => "super",
        "įdėt" => "insert",
        "gaut" => "get",
        "leist" => "allow",
        "panikuot" => "panic",
        "modulis" => "mod",
        "kintamas" => "mut",
        "naujas" => "new",
        "kur" => "where",
        "dėl" => "for",
        "gauk_arba_įterpk_su" => "get_or_insert_with",
        "pagrindinis" => "main",
        "viešas" => "pub",
        "kad" => None?,
        "grąžint" => "return",
        "implementacija" => "impl",
        "nuoroda" => "ref",
        "sutikimas" => "match",
        "jeigu" => "if",
        "kitaip" => "else",
        "pats" => "self",
        "leist" => "let",
        "statinis" => "static",
        "struktūra" => "struct",
        "tikėtis" => "expect",
        "kol" => "while",
        "nauduot" => "use",
        "į" => "into",
        "tiesa" => "true",
        "enumeracija" => "enum",

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
pub fn rudys(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
