// Temporary probe: root build must compile the Ethernaut levels whose
// sources import unprefixed upstream dependencies.
pragma solidity ^0.8.0;

import {NaughtCoin} from "@ethernaut/NaughtCoin.sol";
import {Cashback} from "@ethernaut/Cashback.sol";
import {Forger} from "@ethernaut/Forger.sol";
import {NotOptimisticPortal} from "@ethernaut/NotOptimisticPortal.sol";
import {PuzzleWallet} from "@ethernaut/PuzzleWallet.sol";

contract EthernautImportProbe {
    function touch() public pure returns (bool) {
        return true;
    }
}
