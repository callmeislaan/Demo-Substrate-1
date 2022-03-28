use crate::{mock::*};

use frame_support::{assert_ok};

use crate as pallet_kitty;
use crate::mock as mock;

#[test]
fn should_fn_create_kitty_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let sender = 1u64;

		// ensure funtion work
		assert_ok!(KittyModule::create_kitty(Origin::signed(sender)));

		// ensure storage value
		assert_eq!(KittyModule::kitty_cnt(), 1);

		let event = <frame_system::Pallet<Test>>::events().pop().expect("Expected event").event;

		let kitty_id = KittyModule::kitty_owned(sender).last().unwrap().to_owned();

		assert_eq!(event, mock::Event::from(pallet_kitty::Event::Created(sender, kitty_id)));
	});
}

#[test]
fn shoud_fn_mint_generate_kitty_and_save_to_storage_success() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let kitty_id = KittyModule::mint(&account_id, None, None).unwrap();

		assert_eq!(KittyModule::kitty_cnt(), 1);

		let kitty = KittyModule::kitty(kitty_id);

		assert_eq!(kitty.is_some(), true);

		let bounded_vec = KittyModule::kitty_owned(account_id);

		assert_eq!(bounded_vec.len(), 1);

		assert_eq!(bounded_vec.last(), Some(&kitty_id));

		System::set_block_number(2);

		let kitty_id_2 = KittyModule::mint(&account_id, None, None).unwrap();

		assert_eq!(KittyModule::kitty_cnt(), 2);

		let kitty_2 = KittyModule::kitty(kitty_id_2);

		assert_eq!(kitty_2.is_some(), true);

		let bounded_vec_2 = KittyModule::kitty_owned(account_id);

		assert_eq!(bounded_vec_2.len(), 2);

		assert_eq!(bounded_vec_2.last(), Some(&kitty_id_2));
	})
}

#[test]
fn should_fn_set_price_work() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let owner = Origin::signed(account_id);
		let kitty_id = KittyModule::mint(&account_id, None, None).unwrap();
		let new_price = Some(10);
		assert_ok!(KittyModule::set_price(owner, kitty_id, new_price));

		let event = <frame_system::Pallet<Test>>::events().pop().expect("Expected set price event").event;

		assert_eq!(event, mock::Event::from(pallet_kitty::Event::PriceSet(account_id, kitty_id, new_price)));
	});
}

#[test]
fn shoud_fn_transfer_work() {
	new_test_ext().execute_with(|| {
		let account_id_1 = 1;
		let account_id_2 = 2;
		let onwer_1 = Origin::signed(account_id_1);
		let kitty_id = KittyModule::mint(&account_id_1, None, None).unwrap();
		assert_ok!(KittyModule::transfer(onwer_1, account_id_2, kitty_id));
	});
}

#[test]
fn should_fn_buy_kitty_work() {
	new_test_ext().execute_with(|| {
		let account_id_1 = 1;
		let account_id_2 = 2;
		let owner_1 = Origin::signed(account_id_1);
		let owner_2 = Origin::signed(account_id_2);
		let kitty_id = KittyModule::mint(&account_id_1, None, None).unwrap();
		let _ = KittyModule::set_price(owner_1, kitty_id, Some(10));
		assert_ok!(KittyModule::buy_kitty(owner_2, kitty_id, 10));
	});
}

#[test]
fn should_fn_bread_kitty_work() {
	new_test_ext().execute_with(|| {
		let account_id_1 = 1;
		let owner_1 = Origin::signed(account_id_1);
		let kitty_id_1 = KittyModule::mint(&account_id_1, None, None).unwrap();
		System::set_block_number(2);
		let kitty_id_2 = KittyModule::mint(&account_id_1, None, None).unwrap();
		assert_ok!(KittyModule::breed_kitty(owner_1, kitty_id_1, kitty_id_2));
	});
}
