// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
use frame_support::storage::migration;

#[derive(Decode)]
pub struct OldMintConfig<T: Config> {
	pub open: bool,
	pub fees: MintFees<BalanceOf<T>>,
	pub cooldown: BlockNumberFor<T>,
	pub free_mint_fee_multiplier: MintCount,
}

impl<T> OldMintConfig<T>
where
	T: Config,
{
	fn migrate_to_v5(self) -> MintConfig<BlockNumberFor<T>> {
		MintConfig {
			open: self.open,
			cooldown: self.cooldown,
			free_mint_fee_multiplier: self.free_mint_fee_multiplier,
		}
	}
}

#[derive(Decode)]
pub struct OldTransferConfig<T: Config> {
	pub open: bool,
	pub free_mint_transfer_fee: MintCount,
	pub min_free_mint_transfer: MintCount,
	pub avatar_transfer_fee: BalanceOf<T>,
}

impl<T> OldTransferConfig<T>
where
	T: Config,
{
	fn migrate_to_v5(self) -> TransferConfig {
		TransferConfig {
			open: self.open,
			free_mint_transfer_fee: self.free_mint_transfer_fee,
			min_free_mint_transfer: self.min_free_mint_transfer,
		}
	}
}

#[derive(Decode)]
pub struct OldTradeConfig<T: Config> {
	pub open: bool,
	pub min_fee: BalanceOf<T>,
	pub percent_fee: u8,
}

impl<T> OldTradeConfig<T>
where
	T: Config,
{
	fn migrate_to_v5(self) -> TradeConfig {
		TradeConfig { open: self.open }
	}
}

#[derive(Decode)]
pub struct OldAccountConfig<T: Config> {
	pub storage_upgrade_fee: BalanceOf<T>,
}

#[derive(Decode)]
pub struct OldNftTransferConfig<T: Config> {
	pub open: bool,
	pub prepare_fee: BalanceOf<T>,
}

impl<T> OldNftTransferConfig<T>
where
	T: Config,
{
	fn migrate_to_v5(self) -> NftTransferConfig {
		NftTransferConfig { open: self.open }
	}
}

#[derive(Decode)]
pub struct OldGlobalConfig<T: Config> {
	pub mint: OldMintConfig<T>,
	pub forge: ForgeConfig,
	pub transfer: OldTransferConfig<T>,
	pub trade: OldTradeConfig<T>,
	pub account: OldAccountConfig<T>,
	pub nft_transfer: OldNftTransferConfig<T>,
}

impl<T> OldGlobalConfig<T>
where
	T: Config,
{
	fn migrate_to_v5(self) -> GlobalConfig<BlockNumberFor<T>> {
		GlobalConfig {
			mint: self.mint.migrate_to_v5(),
			forge: self.forge,
			transfer: self.transfer.migrate_to_v5(),
			trade: self.trade.migrate_to_v5(),
			nft_transfer: self.nft_transfer.migrate_to_v5(),
		}
	}
}

#[derive(Decode)]
pub struct OldSeasonStatus {
	pub early: bool,
	pub active: bool,
	pub early_ended: bool,
	pub max_tier_avatars: u32,
}

impl OldSeasonStatus {
	fn migrate_to_v5(self) -> SeasonStatus {
		SeasonStatus {
			season_id: SeasonId::default(),
			early: self.early,
			active: self.active,
			early_ended: self.early_ended,
			max_tier_avatars: self.max_tier_avatars,
		}
	}
}

#[derive(Decode)]
pub struct OldAvatar {
	pub season_id: SeasonId,
	pub dna: Dna,
	pub souls: SoulCount,
}

impl OldAvatar {
	fn migrate_to_v5(self) -> Avatar {
		Avatar {
			season_id: self.season_id,
			encoding: DnaEncoding::V1,
			dna: self.dna,
			souls: self.souls,
		}
	}
}

#[derive(Decode)]
pub struct OldSeason<T: Config> {
	pub name: BoundedVec<u8, ConstU32<100>>,
	pub description: BoundedVec<u8, ConstU32<1_000>>,
	pub early_start: BlockNumberFor<T>,
	pub start: BlockNumberFor<T>,
	pub end: BlockNumberFor<T>,
	pub max_tier_forges: u32,
	pub max_variations: u8,
	pub max_components: u8,
	pub min_sacrifices: SacrificeCount,
	pub max_sacrifices: SacrificeCount,
	pub tiers: BoundedVec<RarityTier, ConstU32<6>>,
	pub single_mint_probs: BoundedVec<RarityPercent, ConstU32<5>>,
	pub batch_mint_probs: BoundedVec<RarityPercent, ConstU32<5>>,
	pub base_prob: RarityPercent,
	pub per_period: BlockNumberFor<T>,
	pub periods: u16,
}

impl<T: Config> OldSeason<T> {
	fn migrate_to_v5(self) -> Season<BlockNumberFor<T>, BalanceOf<T>> {
		Season {
			name: self.name,
			description: self.description,
			early_start: self.early_start,
			start: self.start,
			end: self.end,
			max_tier_forges: self.max_tier_forges,
			max_variations: self.max_variations,
			max_components: self.max_components,
			min_sacrifices: self.min_sacrifices,
			max_sacrifices: self.max_sacrifices,
			tiers: self.tiers,
			single_mint_probs: self.single_mint_probs,
			batch_mint_probs: self.batch_mint_probs,
			base_prob: self.base_prob,
			per_period: self.per_period,
			periods: self.periods,
			trade_filters: Default::default(),
			fee: Fee {
				mint: MintFees {
					one: 550_000_000_000_u64.unique_saturated_into(), // 0.55 BAJU
					three: 500_000_000_000_u64.unique_saturated_into(), // 0.5 BAJU
					six: 450_000_000_000_u64.unique_saturated_into(), // 0.45 BAJU
				},
				transfer_avatar: 1_000_000_000_000_u64.unique_saturated_into(), // 1 BAJU
				buy_minimum: 1_000_000_000_u64.unique_saturated_into(),         // 0.01 BAJU
				buy_percent: 1,                                                 // 1% of sales price
				upgrade_storage: 1_000_000_000_000_u64.unique_saturated_into(), // 1 BAJU
				prepare_avatar: 5_000_000_000_000_u64.unique_saturated_into(),  // 5 BAJU
			},
			mint_logic: LogicGeneration::First,
			forge_logic: LogicGeneration::First,
		}
	}
}

#[derive(Decode)]
pub struct OldAccountInfo<T: Config> {
	pub free_mints: MintCount,
	pub storage_tier: StorageTier,
	pub stats: Stats<BlockNumberFor<T>>,
}

#[frame_support::storage_alias]
pub(crate) type CurrentSeasonId<T: Config> = StorageValue<Pallet<T>, SeasonId, ValueQuery>;

pub struct MigrateToV5<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for MigrateToV5<T> {
	fn on_runtime_upgrade() -> Weight {
		let current_version = Pallet::<T>::current_storage_version();
		let onchain_version = Pallet::<T>::on_chain_storage_version();
		if onchain_version == 4 && current_version == 5 {
			let _ = GlobalConfigs::<T>::translate::<OldGlobalConfig<T>, _>(|old_config| {
				log::info!(target: LOG_TARGET, "Updated GlobalConfig");
				old_config.map(|old| old.migrate_to_v5())
			});

			let _ = CurrentSeasonStatus::<T>::translate::<OldSeasonStatus, _>(|maybe_old_value| {
				maybe_old_value.map(|old_value| {
					log::info!(target: LOG_TARGET, "Migrated current season status");
					let mut new_value = old_value.migrate_to_v5();
					new_value.season_id = CurrentSeasonId::<T>::get();
					new_value
				})
			});

			let owners = migration::storage_iter::<BoundedAvatarIdsOf<T>>(
				<Pallet<T>>::name().as_bytes(),
				b"Owners",
			)
			.drain()
			.filter(|(_owner, avatar_ids)| !avatar_ids.is_empty())
			.map(|(owner, avatar_ids)| (T::AccountId::decode(&mut &owner[..]).unwrap(), avatar_ids))
			.collect::<Vec<_>>();

			let season_id = 1;
			let mut translated_owner_account = 0_u64;
			let mut translated_owner_avatars = 0_u64;
			owners.iter().for_each(|(owner, avatar_ids)| {
				Owners::<T>::insert(owner, season_id, avatar_ids);
				translated_owner_account += 1;
				translated_owner_avatars += avatar_ids.len() as u64;
			});
			log::info!(
				target: LOG_TARGET,
				"Updated {} accounts and {} avatars",
				translated_owner_account,
				translated_owner_avatars,
			);

			let mut translated_trades = 0_u64;
			let trade =
				migration::storage_iter::<BalanceOf<T>>(<Pallet<T>>::name().as_bytes(), b"Trade")
					.drain()
					.map(|(avatar_id, price)| {
						(AvatarIdOf::<T>::decode(&mut &avatar_id[..]).unwrap(), price)
					})
					.collect::<Vec<_>>();
			trade.iter().for_each(|(avatar_id, price)| {
				Trade::<T>::insert(season_id, avatar_id, price);
				translated_trades += 1;
			});
			log::info!(target: LOG_TARGET, "Updated {} avatars in trade", translated_trades);

			let mut translated_avatars = 0_u64;
			Avatars::<T>::translate::<(T::AccountId, OldAvatar), _>(
				|_key, (account_id, old_avatar)| {
					translated_avatars.saturating_inc();
					Some((account_id, old_avatar.migrate_to_v5()))
				},
			);
			log::info!(target: LOG_TARGET, "Updated {} old avatars", translated_avatars);

			let mut translated_account_info = 0_u64;
			let account_info = migration::storage_iter::<OldAccountInfo<T>>(
				<Pallet<T>>::name().as_bytes(),
				b"Accounts",
			)
			.drain()
			.map(|(account_id, OldAccountInfo::<T> { free_mints, storage_tier, stats })| {
				let account_id = T::AccountId::decode(&mut &account_id[..]).unwrap();

				(
					(account_id, PlayerConfig { free_mints }),
					(season_id, PlayerSeasonConfig::<BlockNumberFor<T>> { storage_tier, stats }),
				)
			})
			.collect::<Vec<_>>();
			account_info
				.iter()
				.for_each(|((account_id, config), (season_id, season_config))| {
					PlayerConfigs::<T>::insert(account_id, config);
					PlayerSeasonConfigs::<T>::insert(account_id, season_id, season_config);
					translated_account_info.saturating_inc();
				});
			log::info!(
				target: LOG_TARGET,
				"Updated {} player account info entries",
				translated_account_info
			);

			Seasons::<T>::translate::<OldSeason<T>, _>(|_key, old_value| {
				log::info!(target: LOG_TARGET, "Migrated seasons");
				Some(old_value.migrate_to_v5())
			});

			current_version.put::<Pallet<T>>();
			log::info!(target: LOG_TARGET, "Upgraded storage to version {:?}", current_version);
			T::DbWeight::get().reads_writes(
				3 + 2 * translated_owner_account +
					translated_trades + translated_avatars +
					translated_account_info,
				3 + 2 * translated_owner_account +
					translated_trades + translated_avatars +
					2 * translated_account_info,
			)
		} else {
			log::info!(
				target: LOG_TARGET,
				"Migration did not execute. This probably should be removed"
			);
			T::DbWeight::get().reads(1)
		}
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
		assert_eq!(Pallet::<T>::on_chain_storage_version(), 5);
		assert!(CurrentSeasonStatus::<T>::get().season_id > Zero::zero());

		let mut avatar_ids_from_avatars = Avatars::<T>::iter_keys().collect::<Vec<_>>();
		avatar_ids_from_avatars.sort();
		avatar_ids_from_avatars.dedup();

		let mut avatar_ids_from_owners = Owners::<T>::iter_values().flatten().collect::<Vec<_>>();
		avatar_ids_from_owners.sort();
		avatar_ids_from_owners.dedup();

		// There are 12,976 avatars as of 27/06/2023. But the exact number could be smaller as
		// avatars are forged away. We estimate there should be at least 10,000.
		assert!(avatar_ids_from_avatars.len() > 10_000 && avatar_ids_from_avatars.len() <= 12_976);
		assert!(avatar_ids_from_avatars.len() > 10_000 && avatar_ids_from_owners.len() <= 12_976);
		assert_eq!(avatar_ids_from_avatars, avatar_ids_from_owners);

		// There are 1,245 owners of avatars in storage as of 27/06/2023. But the exact number could
		// change as avatars are traded between accounts.
		// We estimate there should be between 1,200 and 2,000 accounts.
		let mut owners_season_ids = Owners::<T>::iter_keys()
			.filter(|(owner, season_id)| !Owners::<T>::get(owner, season_id).is_empty())
			.map(|(_owner, season_id)| season_id)
			.collect::<Vec<SeasonId>>();
		assert!(owners_season_ids.len() > 1_200 && owners_season_ids.len() < 2_000);

		// Check all owners are migrated with season ID of 1.
		owners_season_ids.sort();
		owners_season_ids.dedup();
		assert_eq!(owners_season_ids.len(), 1);
		assert_eq!(owners_season_ids, vec![1]);

		// Check owner configs
		let player_configs_account_ids = PlayerConfigs::<T>::iter_keys().collect::<Vec<_>>();
		let mut player_season_configs_season_ids = PlayerSeasonConfigs::<T>::iter_keys()
			.map(|(_, season_id)| season_id)
			.collect::<Vec<_>>();
		assert!(
			player_configs_account_ids.len() > 1_200 && player_configs_account_ids.len() < 2_000
		);
		assert!(
			player_season_configs_season_ids.len() > 1_200 &&
				player_season_configs_season_ids.len() < 2_000
		);
		player_season_configs_season_ids.sort();
		player_season_configs_season_ids.dedup();
		assert_eq!(player_season_configs_season_ids.len(), 1);
		assert_eq!(player_season_configs_season_ids, vec![1]);

		// There are 869 avatars in trade as of 27/06/2023. But the exact number could change. we
		// estimate between 800 and 1,000 avatars to be in trade.
		let mut trade_season_ids = Trade::<T>::iter_keys()
			.map(|(season_id, _avatar_id)| season_id)
			.collect::<Vec<SeasonId>>();
		assert!(trade_season_ids.len() > 800 && trade_season_ids.len() < 1_000);

		// Check all trades are migrated with season ID of 1.
		trade_season_ids.sort();
		trade_season_ids.dedup();
		assert_eq!(trade_season_ids.len(), 1);
		assert_eq!(trade_season_ids, vec![1]);

		// Check all migrated avatars are of version 1.
		assert!(Avatars::<T>::iter_values()
			.all(|(_account, avatar)| avatar.encoding == DnaEncoding::V1));

		assert!(Seasons::<T>::get(1).unwrap().trade_filters.is_empty());

		// Check migrated season fees.
		let Season { fee, .. } = Seasons::<T>::get(1).unwrap();
		assert_eq!(fee.mint.one, 550_000_000_000_u64.unique_saturated_into());
		assert_eq!(fee.mint.three, 500_000_000_000_u64.unique_saturated_into());
		assert_eq!(fee.mint.six, 450_000_000_000_u64.unique_saturated_into());
		assert_eq!(fee.transfer_avatar, 1_000_000_000_000_u64.unique_saturated_into());
		assert_eq!(fee.buy_minimum, 1_000_000_000_u64.unique_saturated_into());
		assert_eq!(fee.buy_percent, 1);
		assert_eq!(fee.upgrade_storage, 1_000_000_000_000_u64.unique_saturated_into());
		assert_eq!(fee.prepare_avatar, 5_000_000_000_000_u64.unique_saturated_into());

		Ok(())
	}
}
