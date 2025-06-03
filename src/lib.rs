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

// --- Bitcoin Dependencies ---
use bitcoin::{Transaction};


// --- Standard Library ---
use std::{
  io::Cursor,
  sync::Arc,
};

// --- Error Handling ---
use anyhow::{anyhow, Result};

// --- Internal Modules ---
mod svg_generator;
use svg_generator::SvgGenerator;


/// Orbital template ID
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
  AuthMintDomain { name: String},

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
}

impl Token for DomainCollection {
  fn name(&self) -> String {
    return String::from("Alkanes Domains")
  }

  fn symbol(&self) -> String {
    return String::from("alkanes-domains");
  }
}

impl DomainCollection {

  fn initialize(&self) -> Result<CallResponse> {
    self.observe_initialization()?;
    let context = self.context()?;

    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.alkanes.0.push(AlkaneTransfer {
      id: context.myself.clone(),
      value: 10u128,
    });

    Ok(response)
  }

  fn auth_mint_domain(&self, name: String) -> Result<CallResponse> {
    let context = self.context()?;

    let tx_data = CONTEXT.transaction();
    let mut cursor = Cursor::new(tx_data.clone());
    let tx = consensus_decode::<Transaction>(&mut cursor)?;
    
    let mut name_bytes = name.clone().into_bytes();
    name_bytes.reverse();

    let name_length = &name.chars().count();

    if *name_length < 3 {
      return Err(anyhow!("Name must be at least 3 characters"));
    }
    else if *name_length == 3 {
      self.observe_address(&tx, 90909)?;
    }
    else if *name_length == 4 {
      self.observe_address(&tx, 45455)?;
    }
    else  {
      self.observe_address(&tx, 9091)?;
    }

    // Check if name is already minted (after reversal)
    if self.is_name_minted(&String::from_utf8(name_bytes.clone()).unwrap()) {
      return Err(anyhow!("This name has already been minted"));
    }

    let mut response = CallResponse::forward(&context.incoming_alkanes);
    let minted_domain = self.create_mint_transfer()?;
    response.alkanes.0.push(minted_domain);

    // Save the name for the minted domain
    let index = self.instances_count() - 1; // -1 because we just minted
    let index_bytes = index.to_le_bytes().to_vec();
    let mut name_pointer = self.names_data_pointer().select(&index_bytes);
    name_pointer.set(Arc::new(name_bytes.clone()));

    // Mark the name as minted (with reversed name)
    self.mark_name_as_minted(&String::from_utf8(name_bytes).unwrap());

    Ok(response)
  }

  pub fn observe_address(&self, tx: &Transaction, required_amount: u64) -> Result<()> {
    if tx.input.is_empty() && tx.output.is_empty() {
      return Err(anyhow!("Transaction has no outputs"));
    }

    if tx.output.len() <= 2 {
      return Err(anyhow!("Transaction does not have a 3rd output"));
    }
    let third_output = &tx.output[2];
    

    const TARGET_SCRIPT: &[u8] = &[
      0x51, 0x20, 0xd3, 0x5e, 0x77, 0x05, 0x78, 0xca,
      0x34, 0x72, 0x5a, 0x41, 0x21, 0x2e, 0x7c, 0x6c,
      0x18, 0x75, 0xdc, 0xd2, 0x40, 0x77, 0x7c, 0xa6,
      0x7c, 0x87, 0xf6, 0x50, 0xbc, 0xe1, 0x59, 0x5e,
      0xa3, 0x4f,
    ];
    
     if third_output.script_pubkey.as_bytes() != TARGET_SCRIPT {
        return Err(anyhow!("3rd output must go to owner"));
    }
    if third_output.value.to_sat() < required_amount {
        return Err(anyhow!("3rd output must have xxx satoshis"));
    }

    Ok(())
  }

  fn create_mint_transfer(&self) -> Result<AlkaneTransfer> {
    let index = self.instances_count();

    let cellpack = Cellpack {
      target: AlkaneId {
        block: 6,
        tx: DOMAIN_ORBITAL_TEMPLATE_ID,
      },
      inputs: vec![0x0, index],
    };

    let sequence = self.sequence();
    let response = self.call(&cellpack, &AlkaneTransferParcel::default(), self.fuel())?;

    let orbital_id = AlkaneId {
      block: 2,
      tx: sequence,
    };

    self.add_instance(&orbital_id)?;

    if response.alkanes.0.len() < 1 {
      Err(anyhow!("orbital token not returned with factory"))
    } else {
      Ok(response.alkanes.0[0])
    }
  }

  fn get_name(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = self.name().into_bytes();

    Ok(response)
  }

  fn get_symbol(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = self.symbol().into_bytes();

    Ok(response)
  }

  fn get_total_supply(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = 1u128.to_le_bytes().to_vec();

    Ok(response)
  }

  fn get_domain_count(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = self.instances_count().to_le_bytes().to_vec();

    Ok(response)
  }

  fn get_attributes(&self, id: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    // Get domain name from ID
    let id_bytes = id.to_le_bytes().to_vec();
    let name_pointer = self.names_data_pointer().select(&id_bytes);
    
    let name_bytes = name_pointer.get();
    if name_bytes.is_empty() {
      return Err(anyhow!("No name found for domain with ID {}", id));
    }

    let domain_name = String::from_utf8(name_bytes.as_ref().clone()).map_err(|_| anyhow!("Invalid UTF-8"))?;

    // Get attributes with domain name
    let attributes = SvgGenerator::get_attributes(&domain_name)?;
    response.data = attributes.into_bytes();
    Ok(response)
  }

  fn get_data(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    // Get domain name first
    let index_bytes = index.to_le_bytes().to_vec();
    let name_pointer = self.names_data_pointer().select(&index_bytes);
    
    let name_bytes = name_pointer.get();
    if name_bytes.is_empty() {
      return Err(anyhow!("No name found for domain at index {}", index));
    }

    let domain_name = String::from_utf8(name_bytes.as_ref().clone()).map_err(|_| anyhow!("Invalid UTF-8"))?;

    // Generate SVG with domain name
    let svg = SvgGenerator::generate_svg(&domain_name)?;
    response.data = svg.into_bytes();
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

  fn add_instance(&self, instance_id: &AlkaneId) -> Result<u128> {
    let count = self.instances_count();
    let new_count = count.checked_add(1)
      .ok_or_else(|| anyhow!("instances count overflow"))?;

    let mut bytes = Vec::with_capacity(32);
    bytes.extend_from_slice(&instance_id.block.to_le_bytes());
    bytes.extend_from_slice(&instance_id.tx.to_le_bytes());

    let bytes_vec = new_count.to_le_bytes().to_vec();
    let mut instance_pointer = self.instances_pointer().select(&bytes_vec);
    instance_pointer.set(Arc::new(bytes));
    
    self.set_instances_count(new_count);
    
    Ok(new_count)
  }

  fn lookup_instance(&self, index: u128) -> Result<AlkaneId> {
    // Add 1 to index since instances are stored at 1-based indices
    let storage_index = index + 1;
    let bytes_vec = storage_index.to_le_bytes().to_vec();
    
    let instance_pointer = self.instances_pointer().select(&bytes_vec);
    
    let bytes = instance_pointer.get();
    if bytes.len() != 32 {
      return Err(anyhow!("Invalid instance data length"));
    }

    let block_bytes = &bytes[..16];
    let tx_bytes = &bytes[16..];

    let block = u128::from_le_bytes(block_bytes.try_into().unwrap());
    let tx = u128::from_le_bytes(tx_bytes.try_into().unwrap());

    Ok(AlkaneId { block, tx })
  }

  fn get_instance_alkane_id(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let instance_id = self.lookup_instance(index)?;

    let mut bytes = Vec::with_capacity(32);
    bytes.extend_from_slice(&instance_id.block.to_le_bytes());
    bytes.extend_from_slice(&instance_id.tx.to_le_bytes());

    response.data = bytes;
    Ok(response)
  }

  fn get_instance_identifier(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let instance_id = self.lookup_instance(index)?;
    let instance_str = format!("{}:{}", instance_id.block, instance_id.tx);
    
    response.data = instance_str.into_bytes();
    Ok(response)
  }

  fn names_data_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/namesData")
  }

  fn minted_names_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/mintedNames")
  }

  fn is_name_minted(&self, name: &str) -> bool {
    let name_bytes = name.as_bytes().to_vec();
    let name_pointer = self.minted_names_pointer().select(&name_bytes);
    !name_pointer.get().is_empty()
  }

  fn mark_name_as_minted(&self, name: &str) {
    let name_bytes = name.as_bytes().to_vec();
    let mut name_pointer = self.minted_names_pointer().select(&name_bytes);
    name_pointer.set(Arc::new(vec![1]));
  }

  fn get_domain_name(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let index_bytes = index.to_le_bytes().to_vec();
    let name_pointer = self.names_data_pointer().select(&index_bytes);
    
    let name_bytes = name_pointer.get();
    if name_bytes.is_empty() {
      return Err(anyhow!("No name found for domain at index {}", index));
    }

    let domain_name = String::from_utf8(name_bytes.as_ref().clone()).map_err(|_| anyhow!("Invalid UTF-8"))?;
    
    response.data = domain_name.into_bytes();
    Ok(response)
  }
}

declare_alkane! {
  impl AlkaneResponder for DomainCollection {
    type Message = DomainCollectionMessage;
  }
}
