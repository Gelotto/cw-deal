# cw-deal
CosmWasm smart contract that facilitates the creation and execution of deals
involving arbitrary mixtures of on-chain and off-chain assets. Each deal
supports two or more parties, each contributing one or more assets offered in
consideration. The API of the contract is designed around an offer/counter-offer
workflow, using a combination of structured and unstructured data, enabling
"version control" functionality, whereby parties may review the history of
changes. Upon execution, the resulting state of the smart contract serves as a
robust facsimile of a "real world" contract that may be instrumental in
off-chain business workflows and litigation.