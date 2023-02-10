use frame_support::{assert_ok, BoundedVec};

use crate::{mock::*, Event};

#[test]
fn given_normal_conditions_when_deploy_sierra_program_then_it_works() {
	new_test_ext().execute_with(|| {
		let deployer_account = 1;
		let deployer_origin = RuntimeOrigin::signed(deployer_account);
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let sierra_code = BoundedVec::truncate_from(vec![1, 2, 3]);
		// Dispatch a signed extrinsic.
		assert_ok!(CairoExecutionEngine::deploy_sierra_program(
			deployer_origin,
			sierra_code.clone()
		));

		// Get sierra program id
		let sierra_program_id =
			CairoExecutionEngine::gen_sierra_program_id(&deployer_account, &sierra_code).unwrap();

		// Read pallet storage and assert an expected result.
		let sierra_program = CairoExecutionEngine::sierra_programs(sierra_program_id).unwrap();
		assert_eq!(sierra_program.code, sierra_code);
		// Assert that the correct event was deposited
		System::assert_last_event(
			Event::SierraProgramDeployed { deployer_account, id: sierra_program_id }.into(),
		);
	});
}