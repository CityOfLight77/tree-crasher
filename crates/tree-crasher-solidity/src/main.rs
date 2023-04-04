use anyhow::Result;

fn main() -> Result<()> {
    tree_crasher::main(
        tree_sitter_solidity::language_solidity(),
        tree_sitter_solidity::TYPESCRIPT_NODE_TYPES,
    )
}
