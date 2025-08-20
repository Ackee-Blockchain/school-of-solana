import { createCodamaConfig } from 'gill'

export default createCodamaConfig({
  clientJs: 'anchor/src/client/js/generated',
  idl: 'target/idl/ticketregistry.json',
})
