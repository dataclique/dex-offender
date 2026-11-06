// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "forge-std/Vm.sol";

import {Preservation, LibraryContract} from "@ethernaut/Preservation.sol";

contract PreservationTest is Test {
    address public offender = 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720;
    Preservation public target;

    // this function is run before each test
    function setUp() public {
        vm.prank(address(0)); // sets msg.sender for the next call
        LibraryContract lib1 = new LibraryContract();
        LibraryContract lib2 = new LibraryContract();
        target = new Preservation(address(lib1), address(lib2));
    }

    function testPreservation() public {
        target.setFirstTime(uint256(uint160(offender)));
        target.setSecondTime(uint256(uint160(offender)));
    }
}
