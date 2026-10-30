// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// Minimal Chainlink AggregatorV3 mock for local harness planting at the
/// canonical mainnet ETH/USD feed address. Returns a fixed 2000.00000000
/// USD price on every round.
contract MockEthUsdFeed {
    uint80 public constant latestRound = 1;
    int256 public constant answer = 200000000000;
    uint256 public constant startedAt = 1;
    uint256 public constant updatedAt = 1;

    function decimals() external pure returns (uint8) {
        return 8;
    }

    function description() external pure returns (string memory) {
        return "ETH / USD";
    }

    function version() external pure returns (uint256) {
        return 1;
    }

    function getRoundData(
        uint80
    )
        external
        view
        returns (uint80, int256, uint256, uint256, uint80)
    {
        return (latestRound, answer, startedAt, updatedAt, latestRound);
    }

    function latestRoundData()
        external
        view
        returns (uint80, int256, uint256, uint256, uint80)
    {
        return (latestRound, answer, startedAt, updatedAt, latestRound);
    }
}
