// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "forge-std/Vm.sol";

import {Shop} from "@ethernaut/Shop.sol";
import {ShopExploit} from "../../contracts/ethernaut/ShopExploit.sol";

contract GatekeeperOneTest is Test {
    address public offender = 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720;
    Shop target;

    constructor() {
        target = new Shop();
    }

    function testFuzzShop(uint32 threshold) public {
        vm.startPrank(offender);
        ShopExploit exploit = new ShopExploit(threshold);
        vm.expectRevert();
        exploit.buy{gas: 1_000_000}(target);
    }
}
