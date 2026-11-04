// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "forge-std/Vm.sol";

import {GatekeeperTwo} from "@ethernaut/GatekeeperTwo.sol";
import {GatekeeperTwoExploit} from "../../contracts/ethernaut/GatekeeperTwoExploit.sol";


contract GatekeeperOneTest is Test {
    address public offender = 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720;
    GatekeeperTwo target;

    constructor() {
        target = new GatekeeperTwo();
    }

    function testGatekeeperTwoExploit() public {
        vm.startPrank(offender);
        new GatekeeperTwoExploit(target);
    }
}
