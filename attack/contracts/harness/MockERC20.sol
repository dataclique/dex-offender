// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// Minimal ERC20 mock for planting canonical mainnet token addresses on
/// a local anvil. Storage layout is fixed and documented for
/// anvil_setStorageAt seeding:
///   slot 0: balanceOf mapping
///   slot 1: allowance mapping
///   slot 2: totalSupply
///
/// Deliberately emits no ERC20 events: the public mapping names (and thus
/// the storage order) are load-bearing for slot-exact seeding and the
/// balanceOf getter the harness reads, while no consumer subscribes to
/// Transfer/Approval on the mock. forge's missing-events-access-control
/// lint is therefore a known, accepted finding for this file — not an
/// oversight.
contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint256 public totalSupply;

    function name() external pure returns (string memory) {
        return "Mock";
    }

    function symbol() external pure returns (string memory) {
        return "MCK";
    }

    function decimals() external pure returns (uint8) {
        return 18;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "insufficient balance");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(
        address from,
        address to,
        uint256 amount
    ) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "insufficient allowance");
        require(balanceOf[from] >= amount, "insufficient balance");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}
