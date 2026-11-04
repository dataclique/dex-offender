// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "forge-std/Vm.sol";

import {GatekeeperOne} from "@ethernaut/GatekeeperOne.sol";
import {GatekeeperOneExploit} from "../../contracts/ethernaut/GatekeeperOneExploit.sol";


contract GatekeeperOneTest is Test {
    address public offender = 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720;
    // GatekeeperOne target;
    //
    // constructor() {
    //     target = new GatekeeperOne();
    // }
    //
    // function testFuzzFindGasParam(uint16 offset) public {
    //     vm.startPrank(offender);
    //     vm.expectRevert();
    //     new GatekeeperOneExploit(target, offset);
    // }

    function testGateKey() public {
        bytes8 _gateKey = bytes8(0x0123456700009720);

        console.logBytes8(_gateKey);

        console.log("LHS 1: ", uint32(uint64(_gateKey)));
        console.log("RHS 1: ", uint16(uint64(_gateKey)));

        console.log("LHS 2: ", uint32(uint64(_gateKey)));
        console.log("RHS 2: ", uint64(_gateKey));

        console.log("LHS 3: ", uint32(uint64(_gateKey)));
        console.log("RHS 3: ", uint16(uint160(offender)));

        assertTrue(uint32(uint64(_gateKey)) == uint16(uint64(_gateKey)), "First condition must pass");
        assertTrue(uint32(uint64(_gateKey)) != uint64(_gateKey), "Second condition must pass");
        assertTrue(uint32(uint64(_gateKey)) == uint16(uint160(offender)), "Third condition must pass");
    }
}
