//! Per-level hints for the practice runner.
//!
//! `hint!()` inside an `Exploit::attack` implementation prints the level's
//! upstream hint (Ethernaut ships one per level; Damn Vulnerable DeFi and
//! Paradigm do not). Nothing is printed unless the player asks for it.

/// Module stem → upstream hint text, from the pinned ethernaut gamedata.
const HINTS: &[(&str, &str)] = &[
    (
        "hack01_fallback",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/fallback.md"
        ),
    ),
    (
        "hack02_fallout",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/fallout.md"
        ),
    ),
    (
        "hack03_coin_flip",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/coinflip.md"
        ),
    ),
    (
        "hack04_telephone",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/telephone.md"
        ),
    ),
    (
        "hack05_token",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/token.md"
        ),
    ),
    (
        "hack06_delegate",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/delegate.md"
        ),
    ),
    (
        "hack07_force",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/force.md"
        ),
    ),
    (
        "hack08_vault",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/vault.md"
        ),
    ),
    (
        "hack09_king",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/king.md"
        ),
    ),
    (
        "hack10_reentrancy",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/reentrancy.md"
        ),
    ),
    (
        "hack11_elevator",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/elevator.md"
        ),
    ),
    (
        "hack12_privacy",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/privacy.md"
        ),
    ),
    (
        "hack13_gatekeeper_one",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/gatekeeper1.md"
        ),
    ),
    (
        "hack14_gatekeeper_two",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/gatekeeper2.md"
        ),
    ),
    (
        "hack15_naught_coin",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/naughtcoin.md"
        ),
    ),
    (
        "hack16_preservation",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/preservation.md"
        ),
    ),
    (
        "hack17_recovery",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/recovery.md"
        ),
    ),
    (
        "hack18_magic_number",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/magicnum.md"
        ),
    ),
    (
        "hack19_alien_codex",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/aliencodex.md"
        ),
    ),
    (
        "hack20_denial",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/denial.md"
        ),
    ),
    (
        "hack21_shop",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/shop.md"
        ),
    ),
    (
        "hack22_dex",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/dex.md"
        ),
    ),
    (
        "hack23_dex_two",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/dex2.md"
        ),
    ),
    (
        "hack24_puzzle_wallet",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/puzzle_wallet.md"
        ),
    ),
    (
        "hack25_motorbike",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/motorbike.md"
        ),
    ),
    (
        "hack26_double_entry_point",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/doubleentrypoint.md"
        ),
    ),
    (
        "hack27_good_samaritan",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/goodsamaritan.md"
        ),
    ),
    (
        "hack28_gatekeeper_three",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/gatekeeper3.md"
        ),
    ),
    (
        "hack29_switch",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/switch.md"
        ),
    ),
    (
        "hack30_higher_order",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/higherorder.md"
        ),
    ),
    (
        "hack31_stake",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/stake.md"
        ),
    ),
    (
        "hack32_impersonator",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/impersonator.md"
        ),
    ),
    (
        "hack33_magic_animal_carousel",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/magicanimalcarousel.md"
        ),
    ),
    (
        "hack34_bet_house",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/bet_house.md"
        ),
    ),
    (
        "hack35_elliptic_token",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/elliptic_token.md"
        ),
    ),
    (
        "hack36_cashback",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/cashback.md"
        ),
    ),
    (
        "hack37_impersonator_two",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/impersonator_two.md"
        ),
    ),
    (
        "hack38_unique_nft",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/uniquenft.md"
        ),
    ),
    (
        "hack39_forger",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/forger.md"
        ),
    ),
    (
        "hack40_not_optimistic_portal",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/not_optimistic_portal.md"
        ),
    ),
];

/// The upstream hint for the module at `path`, when one exists.
pub fn hint_for(path: &str) -> Option<&'static str> {
    HINTS
        .iter()
        .find(|(module, _)| path.contains(module))
        .map(|(_, text)| *text)
}

/// Print the calling level's hint (used by the `hint!` macro).
pub fn print_for(path: &str) {
    match hint_for(path) {
        Some(text) => println!("\n💡 hint:\n{}\n", render_markdown(text)),
        None => println!("\n💡 no hint registered for this level"),
    }
}

/// Render embedded markdown for the terminal: links become
/// `label (url)` (or a bare url), emphasis/code markers are stripped,
/// heading markers flattened and `&nbsp;` dropped. Bullet and numbered
/// lists keep their plain-text shape.
pub fn render_markdown(input: &str) -> String {
    let mut rendered = String::new();
    for line in input.lines() {
        let line = line.trim_end();
        if line.trim() == "&nbsp;" || line.is_empty() {
            rendered.push('\n');
            continue;
        }
        let mut inline = render_links(line);
        inline = inline.replace("**", "");
        inline = inline.replace('`', "");
        let flattened = inline.trim_start_matches('#').trim_start();
        rendered.push_str(flattened.trim_end());
        rendered.push('\n');
    }
    rendered.trim().to_string()
}

/// Convert `[label](url)` to `label (url)`; keep bare urls as-is.
fn render_links(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut out = String::new();
    let mut index = 0;
    while index < chars.len() {
        if chars[index] == '[' {
            if let Some((label, url, next)) = parse_link(&chars, index) {
                if label == url {
                    out.push_str(&url);
                } else {
                    out.push_str(&label);
                    out.push_str(" (");
                    out.push_str(&url);
                    out.push(')');
                }
                index = next;
                continue;
            }
        }
        out.push(chars[index]);
        index += 1;
    }
    out
}

/// Parse `[label](url)` starting at `start` (which points at '[').
fn parse_link(chars: &[char], start: usize) -> Option<(String, String, usize)> {
    let close =
        chars.iter().skip(start + 1).position(|&c| c == ']')? + start + 1;
    if close + 1 >= chars.len() || chars[close + 1] != '(' {
        return None;
    }
    let end = chars.iter().skip(close + 2).position(|&c| c == ')')? + close + 2;
    let label: String = chars[start + 1..close].iter().collect();
    let url: String = chars[close + 2..end].iter().collect();
    Some((label, url, end + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_links_become_terminal_text() {
        let rendered = render_markdown(
            "See [Fallback](https://ethernaut.openzeppelin.com) and \
             <https://example.com>\n\n**Bold** and `code` stay plain.",
        );
        assert!(
            rendered.contains("Fallback (https://ethernaut.openzeppelin.com)")
        );
        assert!(rendered.contains("<https://example.com>"));
        assert!(rendered.contains("Bold and code stay plain."));
    }

    #[test]
    fn markdown_headings_and_separators_flatten() {
        let rendered =
            render_markdown("## Congratulations\n&nbsp;\n* keep going");
        assert!(rendered.starts_with("Congratulations\n\n* keep going"));
        assert!(!rendered.contains("##"));
        assert!(!rendered.contains("&nbsp;"));
    }

    #[test]
    fn hints_resolve_by_module_path() {
        assert!(
            hint_for("attack/src/ethereum/ethernaut/hack01_fallback.rs")
                .expect("fallback hint")
                .contains("Things that might help")
        );
        assert!(
            hint_for(
                "attack/src/ethereum/damn_vulnerable_defi/hack01_unstoppable.rs"
            )
            .is_none()
        );
    }
}
