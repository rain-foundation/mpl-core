//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::ExternalPluginAdapterKey;
#[cfg(feature = "anchor")]
use anchor_lang::prelude::{AnchorDeserialize, AnchorSerialize};
#[cfg(not(feature = "anchor"))]
use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct WriteExternalPluginAdapterDataV1 {
    /// The address of the asset
    pub asset: solana_program::pubkey::Pubkey,
    /// The collection to which the asset belongs
    pub collection: Option<solana_program::pubkey::Pubkey>,
    /// The account paying for the storage fees
    pub payer: solana_program::pubkey::Pubkey,
    /// The Data Authority of the External Plugin Adapter
    pub authority: Option<solana_program::pubkey::Pubkey>,
    /// The buffer to write to the external plugin
    pub buffer: Option<solana_program::pubkey::Pubkey>,
    /// The system program
    pub system_program: solana_program::pubkey::Pubkey,
    /// The SPL Noop Program
    pub log_wrapper: Option<solana_program::pubkey::Pubkey>,
}

impl WriteExternalPluginAdapterDataV1 {
    pub fn instruction(
        &self,
        args: WriteExternalPluginAdapterDataV1InstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: WriteExternalPluginAdapterDataV1InstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.asset, false,
        ));
        if let Some(collection) = self.collection {
            accounts.push(solana_program::instruction::AccountMeta::new(
                collection, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        if let Some(authority) = self.authority {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                authority, true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_ID,
                false,
            ));
        }
        if let Some(buffer) = self.buffer {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                buffer, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        if let Some(log_wrapper) = self.log_wrapper {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                log_wrapper,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let mut data = WriteExternalPluginAdapterDataV1InstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::MPL_CORE_ID,
            accounts,
            data,
        }
    }
}

#[cfg_attr(not(feature = "anchor"), derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "anchor", derive(AnchorSerialize, AnchorDeserialize))]
pub struct WriteExternalPluginAdapterDataV1InstructionData {
    discriminator: u8,
}

impl WriteExternalPluginAdapterDataV1InstructionData {
    pub fn new() -> Self {
        Self { discriminator: 28 }
    }
}

#[cfg_attr(not(feature = "anchor"), derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "anchor", derive(AnchorSerialize, AnchorDeserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WriteExternalPluginAdapterDataV1InstructionArgs {
    pub key: ExternalPluginAdapterKey,
    pub data: Option<Vec<u8>>,
}

/// Instruction builder for `WriteExternalPluginAdapterDataV1`.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[writable, optional]` collection
///   2. `[writable, signer]` payer
///   3. `[signer, optional]` authority
///   4. `[optional]` buffer
///   5. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   6. `[optional]` log_wrapper
#[derive(Default)]
pub struct WriteExternalPluginAdapterDataV1Builder {
    asset: Option<solana_program::pubkey::Pubkey>,
    collection: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    buffer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    log_wrapper: Option<solana_program::pubkey::Pubkey>,
    key: Option<ExternalPluginAdapterKey>,
    data: Option<Vec<u8>>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl WriteExternalPluginAdapterDataV1Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The address of the asset
    #[inline(always)]
    pub fn asset(&mut self, asset: solana_program::pubkey::Pubkey) -> &mut Self {
        self.asset = Some(asset);
        self
    }
    /// `[optional account]`
    /// The collection to which the asset belongs
    #[inline(always)]
    pub fn collection(&mut self, collection: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.collection = collection;
        self
    }
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// `[optional account]`
    /// The Data Authority of the External Plugin Adapter
    #[inline(always)]
    pub fn authority(&mut self, authority: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.authority = authority;
        self
    }
    /// `[optional account]`
    /// The buffer to write to the external plugin
    #[inline(always)]
    pub fn buffer(&mut self, buffer: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.buffer = buffer;
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// The system program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account]`
    /// The SPL Noop Program
    #[inline(always)]
    pub fn log_wrapper(
        &mut self,
        log_wrapper: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.log_wrapper = log_wrapper;
        self
    }
    #[inline(always)]
    pub fn key(&mut self, key: ExternalPluginAdapterKey) -> &mut Self {
        self.key = Some(key);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn data(&mut self, data: Vec<u8>) -> &mut Self {
        self.data = Some(data);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = WriteExternalPluginAdapterDataV1 {
            asset: self.asset.expect("asset is not set"),
            collection: self.collection,
            payer: self.payer.expect("payer is not set"),
            authority: self.authority,
            buffer: self.buffer,
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            log_wrapper: self.log_wrapper,
        };
        let args = WriteExternalPluginAdapterDataV1InstructionArgs {
            key: self.key.clone().expect("key is not set"),
            data: self.data.clone(),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `write_external_plugin_adapter_data_v1` CPI accounts.
pub struct WriteExternalPluginAdapterDataV1CpiAccounts<'a, 'b> {
    /// The address of the asset
    pub asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// The collection to which the asset belongs
    pub collection: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Data Authority of the External Plugin Adapter
    pub authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The buffer to write to the external plugin
    pub buffer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL Noop Program
    pub log_wrapper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `write_external_plugin_adapter_data_v1` CPI instruction.
pub struct WriteExternalPluginAdapterDataV1Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the asset
    pub asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// The collection to which the asset belongs
    pub collection: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Data Authority of the External Plugin Adapter
    pub authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The buffer to write to the external plugin
    pub buffer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL Noop Program
    pub log_wrapper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: WriteExternalPluginAdapterDataV1InstructionArgs,
}

impl<'a, 'b> WriteExternalPluginAdapterDataV1Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: WriteExternalPluginAdapterDataV1CpiAccounts<'a, 'b>,
        args: WriteExternalPluginAdapterDataV1InstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            asset: accounts.asset,
            collection: accounts.collection,
            payer: accounts.payer,
            authority: accounts.authority,
            buffer: accounts.buffer,
            system_program: accounts.system_program,
            log_wrapper: accounts.log_wrapper,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.asset.key,
            false,
        ));
        if let Some(collection) = self.collection {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *collection.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        if let Some(authority) = self.authority {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *authority.key,
                true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_ID,
                false,
            ));
        }
        if let Some(buffer) = self.buffer {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *buffer.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        if let Some(log_wrapper) = self.log_wrapper {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *log_wrapper.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = WriteExternalPluginAdapterDataV1InstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_CORE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.asset.clone());
        if let Some(collection) = self.collection {
            account_infos.push(collection.clone());
        }
        account_infos.push(self.payer.clone());
        if let Some(authority) = self.authority {
            account_infos.push(authority.clone());
        }
        if let Some(buffer) = self.buffer {
            account_infos.push(buffer.clone());
        }
        account_infos.push(self.system_program.clone());
        if let Some(log_wrapper) = self.log_wrapper {
            account_infos.push(log_wrapper.clone());
        }
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `WriteExternalPluginAdapterDataV1` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[writable, optional]` collection
///   2. `[writable, signer]` payer
///   3. `[signer, optional]` authority
///   4. `[optional]` buffer
///   5. `[]` system_program
///   6. `[optional]` log_wrapper
pub struct WriteExternalPluginAdapterDataV1CpiBuilder<'a, 'b> {
    instruction: Box<WriteExternalPluginAdapterDataV1CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> WriteExternalPluginAdapterDataV1CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(WriteExternalPluginAdapterDataV1CpiBuilderInstruction {
            __program: program,
            asset: None,
            collection: None,
            payer: None,
            authority: None,
            buffer: None,
            system_program: None,
            log_wrapper: None,
            key: None,
            data: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The address of the asset
    #[inline(always)]
    pub fn asset(&mut self, asset: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.asset = Some(asset);
        self
    }
    /// `[optional account]`
    /// The collection to which the asset belongs
    #[inline(always)]
    pub fn collection(
        &mut self,
        collection: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.collection = collection;
        self
    }
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// `[optional account]`
    /// The Data Authority of the External Plugin Adapter
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.authority = authority;
        self
    }
    /// `[optional account]`
    /// The buffer to write to the external plugin
    #[inline(always)]
    pub fn buffer(
        &mut self,
        buffer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.buffer = buffer;
        self
    }
    /// The system program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    /// `[optional account]`
    /// The SPL Noop Program
    #[inline(always)]
    pub fn log_wrapper(
        &mut self,
        log_wrapper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.log_wrapper = log_wrapper;
        self
    }
    #[inline(always)]
    pub fn key(&mut self, key: ExternalPluginAdapterKey) -> &mut Self {
        self.instruction.key = Some(key);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn data(&mut self, data: Vec<u8>) -> &mut Self {
        self.instruction.data = Some(data);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = WriteExternalPluginAdapterDataV1InstructionArgs {
            key: self.instruction.key.clone().expect("key is not set"),
            data: self.instruction.data.clone(),
        };
        let instruction = WriteExternalPluginAdapterDataV1Cpi {
            __program: self.instruction.__program,

            asset: self.instruction.asset.expect("asset is not set"),

            collection: self.instruction.collection,

            payer: self.instruction.payer.expect("payer is not set"),

            authority: self.instruction.authority,

            buffer: self.instruction.buffer,

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            log_wrapper: self.instruction.log_wrapper,
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct WriteExternalPluginAdapterDataV1CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    asset: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    buffer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    log_wrapper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    key: Option<ExternalPluginAdapterKey>,
    data: Option<Vec<u8>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}