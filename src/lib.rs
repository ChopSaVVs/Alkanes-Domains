// --- Metashrew Support ---
use metashrew_support::{
  compat::to_arraybuffer_layout,
  index_pointer::KeyValuePointer,
  utils::consensus_decode,
};

// --- Alkanes Runtime / Support ---
use alkanes_runtime::{
  declare_alkane,
  message::MessageDispatch,
  runtime::AlkaneResponder,
  storage::StoragePointer,
  token::Token,
};

use alkanes_support::{
  cellpack::Cellpack,
  id::AlkaneId,
  parcel::{AlkaneTransfer, AlkaneTransferParcel},
  response::CallResponse,
};

use bitcoin::Transaction;

use std::{
  io::Cursor,
  sync::Arc,
};

use anyhow::{anyhow, Result};

mod svg_generator;
use svg_generator::SvgGenerator;

const DOMAIN_ORBITAL_TEMPLATE_ID: u128 = 0x3410;

pub struct ContextHandle(());
impl AlkaneResponder for ContextHandle {}
pub const CONTEXT: ContextHandle = ContextHandle(());

#[derive(Default)]
pub struct DomainCollection(());
impl AlkaneResponder for DomainCollection {}

#[derive(MessageDispatch)]
enum DomainCollectionMessage {
  #[opcode(0)]
  Initialize,

  #[opcode(69)]
  AuthMintDomain { name: String },

  #[opcode(70)]
  TransferDomain { index: u128, to: AlkaneId },

  #[opcode(99)]
  #[returns(String)]
  GetName,

  #[opcode(100)]
  #[returns(String)]
  GetSymbol,

  #[opcode(101)]
  #[returns(u128)]
  GetTotalSupply,

  #[opcode(102)]
  #[returns(u128)]
  GetDomainCount,

  #[opcode(999)]
  #[returns(String)]
  GetAttributes { index: u128 },

  #[opcode(1000)]
  #[returns(Vec<u8>)]
  GetData { index: u128 },

  #[opcode(1001)]
  #[returns(Vec<u8>)]
  GetInstanceAlkaneId { index: u128 },

  #[opcode(1002)]
  #[returns(String)]
  GetInstanceIdentifier { index: u128 },

  #[opcode(1003)]
  #[returns(String)]
  GetDomainName { index: u128 },

  #[opcode(777)]
  UpdateOwnerScript { new_script: Vec<u8> },
}

static mut OWNER_SCRIPT: Option<Vec<u8>> = None;

impl Token for DomainCollection {
  fn name(&self) -> String {
    String::from("Alkanes Domains")
  }
  fn symbol(&self) -> String {
    String::from("alkanes-domains")
  }
}

impl DomainCollection {
  fn initialize(&self) -> Result<CallResponse> {
    self.observe_initialization()?;
    Ok(CallResponse::default())
  }

  fn auth_mint_domain(&self, name: String) -> Result<CallResponse> {
    let context = self.context()?;
    let tx_data = CONTEXT.transaction();
    let tx = consensus_decode::<Transaction>(&mut Cursor::new(tx_data))?;

    let name_len = name.chars().count();
    if name_len < 3 {
      return Err(anyhow!("Name must be at least 3 characters"));
    }

    let required_sats = match name_len {
      3 => 90909,
      4 => 45455,
      _ => 9091,
    };

    self.observe_output(&tx, required_sats)?;

    if self.is_name_minted(&name) {
      return Err(anyhow!("This name has already been minted"));
    }

    let mut response = CallResponse::forward(&context.incoming_alkanes);
    let minted = self.create_mint_transfer()?;
    response.alkanes.0.push(minted);

    let index = self.instances_count();
    self.names_data_pointer().select(&index.to_le_bytes()).set(Arc::new(name.clone().into_bytes()));
    self.mark_name_as_minted(&name);
    self.add_instance(&AlkaneId { block: 6, tx: index })?;

    Ok(response)
  }

  fn transfer_domain(&self, index: u128, to: AlkaneId) -> Result<CallResponse> {
    let context = self.context()?;
    let caller_id = context.sender.ok_or_else(|| anyhow!("Missing sender context"))?;
    let current_owner = self.lookup_instance(index)?;

    if caller_id != current_owner {
      return Err(anyhow!("Only the current owner can transfer this domain"));
    }

    let mut bytes = vec![];
    bytes.extend_from_slice(&to.block.to_le_bytes());
    bytes.extend_from_slice(&to.tx.to_le_bytes());
    self.instances_pointer().select(&index.to_le_bytes()).set(Arc::new(bytes));

    Ok(CallResponse::default())
  }));
    Ok(CallResponse::default())
  }

  fn observe_output(&self, tx: &Transaction, required: u64) -> Result<()> {
    if tx.output.len() <= 2 {
      return Err(anyhow!("Transaction does not have 3rd output"));
    }
    let output = &tx.output[2];
    let target = unsafe { OWNER_SCRIPT.as_ref().ok_or(anyhow!("Owner script not set"))? };

    if output.script_pubkey.as_bytes() != target {
      return Err(anyhow!("3rd output must go to authorized address"));
    }
    if output.value.to_sat() < required {
      return Err(anyhow!("3rd output value too low"));
    }
    Ok(())
  }

  fn update_owner_script(&self, new_script: Vec<u8>) -> Result<CallResponse> {
    unsafe { OWNER_SCRIPT = Some(new_script); }
    Ok(CallResponse::default())
  }

  fn create_mint_transfer(&self) -> Result<AlkaneTransfer> {
    let index = self.instances_count();
    let cellpack = Cellpack { target: AlkaneId { block: 6, tx: DOMAIN_ORBITAL_TEMPLATE_ID }, inputs: vec![0, index] };
    let response = self.call(&cellpack, &AlkaneTransferParcel::default(), self.fuel())?;
    self.set_instances_count(index + 1);
    response.alkanes.0.into_iter().next().ok_or_else(|| anyhow!("Mint failed"))
  }

  fn get_total_supply(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);
    response.data = self.instances_count().to_le_bytes().to_vec();
    Ok(response)
  }

  fn instances_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/instances")
  }
  fn instances_count(&self) -> u128 {
    self.instances_pointer().get_value::<u128>()
  }
  fn set_instances_count(&self, count: u128) {
    self.instances_pointer().set_value::<u128>(count);
  }
  fn add_instance(&self, id: &AlkaneId) -> Result<u128> {
    let count = self.instances_count();
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&id.block.to_le_bytes());
    bytes.extend_from_slice(&id.tx.to_le_bytes());
    self.instances_pointer().select(&count.to_le_bytes()).set(Arc::new(bytes));
    self.set_instances_count(count + 1);
    Ok(count)
  }
  fn lookup_instance(&self, index: u128) -> Result<AlkaneId> {
    let bytes = self.instances_pointer().select(&index.to_le_bytes()).get();
    if bytes.len() != 32 {
      return Err(anyhow!("Invalid instance data length"));
    }
    Ok(AlkaneId {
      block: u128::from_le_bytes(bytes[..16].try_into().unwrap()),
      tx: u128::from_le_bytes(bytes[16..].try_into().unwrap()),
    })
  }
  fn names_data_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/namesData")
  }
  fn minted_names_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/mintedNames")
  }
  fn is_name_minted(&self, name: &str) -> bool {
    !self.minted_names_pointer().select(&name.as_bytes().to_vec()).get().is_empty()
  }
  fn mark_name_as_minted(&self, name: &str) {
    self.minted_names_pointer().select(&name.as_bytes().to_vec()).set(Arc::new(vec![1]));
  }
}

declare_alkane! {
  impl AlkaneResponder for DomainCollection {
    type Message = DomainCollectionMessage;
  }
}
