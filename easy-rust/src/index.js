const tx = {
    "nonce": "0x0",                  // Nonce in hexadecimal format
    "gasPrice": "0x09184e72a000",    // Gas price in Wei (hexadecimal)
    "gasLimit": "0x30000",           // Gas limit in Wei (hexadecimal)
    "to": "0xRecipientAddress",      // Recipient's Ethereum address (hexadecimal)
    "value": "0x12345",              // Value to be sent in Wei (hexadecimal)
    "data": "0x",                    // Input data for contract interactions (hexadecimal)
    "v": "0x1c",                     // Recovery ID for the digital signature (hexadecimal)
    "r": "0x7c1ec827d6a215e24f...",  // R component of the signature (hexadecimal)
    "s": "0x77fb11b3a54b7f3d10f..."   // S component of the signature (hexadecimal)
  }

  const block = []

  for (let i=0; i<200; i++) {
    block.push(tx)
}

console.log(block);