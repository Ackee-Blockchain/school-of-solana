import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Ticketregistry } from '../target/types/ticketregistry'
import { LAMPORTS_PER_SOL, Connection, PublicKey } from '@solana/web3.js'
import assert from 'assert'
import BN from 'bn.js'

describe('ticketregistry', () => {
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.ticketregistry as Program<Ticketregistry>
  const eventOrganizer = anchor.web3.Keypair.generate()
  const buyer1 = anchor.web3.Keypair.generate()
  const buyer2 = anchor.web3.Keypair.generate()
  const unauthorizedUser = anchor.web3.Keypair.generate()

  it('Airdrop SOL to all test accounts', async () => {
    // Airdrop SOL to all test accounts
    await airdrop(program.provider.connection, eventOrganizer.publicKey, 5 * LAMPORTS_PER_SOL)
    await airdrop(program.provider.connection, buyer1.publicKey, 5 * LAMPORTS_PER_SOL)
    await airdrop(program.provider.connection, buyer2.publicKey, 5 * LAMPORTS_PER_SOL)
    await airdrop(program.provider.connection, unauthorizedUser.publicKey, 5 * LAMPORTS_PER_SOL)
  })

  describe('Initialize Event', () => {
    it('Should initialize a valid event successfully', async () => {
      const event = {
        name: 'Test Event',
        description: 'Test Description',
        startDate: new BN(Math.floor(Date.now() / 1000) + 7200), // 2 hours from now
        ticketPrice: new BN(0.5 * LAMPORTS_PER_SOL),
        availableTickets: new BN(100),
      }

      await createEvent(program, eventOrganizer, event)
      await verifyEvent(program, event, eventOrganizer)
    })

    it('Should fail when event name is too long', async () => {
      const testEvent = {
        name: 'a'.repeat(31), // MAX_NAME_LENGTH is 30
        description: 'Test Description',
        startDate: new BN(Math.floor(Date.now() / 1000) + 3600),
        ticketPrice: new BN(0.5 * LAMPORTS_PER_SOL),
        availableTickets: new BN(100),
      }

      try {
        await createEvent(program, eventOrganizer, testEvent)

        // If we get here, the event was created successfully - this means the MAX_LEN macro
        // doesn't enforce length limits at runtime, only for space calculation!
        const [eventPda] = getEventPda(program, testEvent.name, eventOrganizer.publicKey)
        const eventAccount = await program.account.event.fetch(eventPda)

        console.log('🚨 MAX_LEN MACRO DOES NOT ENFORCE LENGTH LIMITS! 🚨')
        console.log('Event name:', eventAccount.name)
        console.log('Event name length:', eventAccount.name.length)
        console.log('Expected max length: 30')
        console.log('Actual length:', eventAccount.name.length)

        // This test should fail to demonstrate the issue
        assert(
          eventAccount.name.length <= 30,
          `Event name length (${eventAccount.name.length}) exceeds maximum allowed (30). ` +
            'The #[max_len] macro does not enforce runtime length validation!',
        )
      } catch (error) {
        // This is what we expect - the event creation should fail due to our validation
        assert(error.message.includes('Name too long'))
      }
    })

    it('Should fail when event description is too long', async () => {
      const event = {
        name: 'Valid Name',
        description: 'a'.repeat(301), // MAX_DESCRIPTION_LENGTH is 300
        startDate: new BN(Math.floor(Date.now() / 1000) + 3600),
        ticketPrice: new BN(0.5 * LAMPORTS_PER_SOL),
        availableTickets: new BN(100),
      }

      try {
        await createEvent(program, eventOrganizer, event)

        const [eventPda] = getEventPda(program, event.name, eventOrganizer.publicKey)
        const eventAccount = await program.account.event.fetch(eventPda)

        console.log('🚨 MAX_LEN MACRO DOES NOT ENFORCE LENGTH LIMITS! 🚨')
        console.log('Event description:', eventAccount.description)
        console.log('Event description length:', eventAccount.description.length)
        console.log('Expected max length: 300')
        console.log('Actual length:', eventAccount.description.length)

        // This test should fail to demonstrate the issue
        assert(
          eventAccount.description.length <= 300,
          `Event description length (${eventAccount.description.length}) exceeds maximum allowed (300). ` +
            'The #[max_len] macro does not enforce runtime length validation!',
        )
      } catch (error) {
        assert(error.message.includes('Description too long'))
      }
    })

    it('Should fail when start date is in the past', async () => {
      const event = {
        name: 'Past Event',
        description: 'Test Description',
        startDate: new BN(Math.floor(Date.now() / 1000) - 3600), // 1 hour ago
        ticketPrice: new BN(0.5 * LAMPORTS_PER_SOL),
        availableTickets: new BN(100),
      }

      try {
        await createEvent(program, eventOrganizer, event)

        // If we get here, the event was created successfully - this means our validation failed!
        const [eventPda] = getEventPda(program, event.name, eventOrganizer.publicKey)
        const eventAccount = await program.account.event.fetch(eventPda)

        console.log('🚨 START DATE VALIDATION FAILED! 🚨')
        console.log('Event start date:', eventAccount.startDate.toString())
        console.log('Current timestamp:', Math.floor(Date.now() / 1000))
        console.log('Expected: start date should be > current timestamp')

        // This test should fail to demonstrate the issue
        assert(
          eventAccount.startDate.toNumber() > Math.floor(Date.now() / 1000),
          `Event start date (${eventAccount.startDate.toString()}) is in the past! ` +
            'Start date validation is not working properly!',
        )
      } catch (error) {
        // This is what we expect - the event creation should fail due to our validation
        assert(error.message.includes('Start date is in the past'))
      }
    })

    it('Should fail when available tickets is zero', async () => {
      const event = {
        name: 'No Tickets Event',
        description: 'Test Description',
        startDate: new BN(Math.floor(Date.now() / 1000) + 3600),
        ticketPrice: new BN(0.5 * LAMPORTS_PER_SOL),
        availableTickets: new BN(0),
      }

      try {
        await createEvent(program, eventOrganizer, event)

        // If we get here, the event was created successfully - this means our validation failed!
        const [eventPda] = getEventPda(program, event.name, eventOrganizer.publicKey)
        const eventAccount = await program.account.event.fetch(eventPda)

        console.log('🚨 AVAILABLE TICKETS VALIDATION FAILED! 🚨')
        console.log('Event available tickets:', eventAccount.availableTickets.toString())
        console.log('Expected: available tickets should be > 0')

        // This test should fail to demonstrate the issue
        assert(
          eventAccount.availableTickets.toNumber() > 0,
          `Event available tickets (${eventAccount.availableTickets.toString()}) is zero! ` +
            'Available tickets validation is not working properly!',
        )
      } catch (error) {
        // This is what we expect - the event creation should fail due to our validation
        assert(error.message.includes('Available tickets is too low'))
      }
    })

    it('Should accept event with maximum allowed name and description lengths', async () => {
      const event = {
        name: 'a'.repeat(30), // Exactly MAX_NAME_LENGTH
        description: 'b'.repeat(300), // Exactly MAX_DESCRIPTION_LENGTH
        startDate: new BN(Math.floor(Date.now() / 1000) + 3600),
        ticketPrice: new BN(0.1 * LAMPORTS_PER_SOL),
        availableTickets: new BN(1),
      }

      await createEvent(program, eventOrganizer, event)
      await verifyEvent(program, event, eventOrganizer)
    })
  })

  describe('Buy Ticket', () => {
    let validEvent: TicketRegistryEvent

    beforeEach(async () => {
      // Create a fresh event for each test with short name (max 30 chars)
      const eventName = `Evt${Date.now().toString().slice(-8)}`
      validEvent = {
        name: eventName,
        description: 'Buyable Event',
        startDate: new BN(Math.floor(Date.now() / 1000) + 7200), // 2 hours from now
        ticketPrice: new BN(0.1 * LAMPORTS_PER_SOL),
        availableTickets: new BN(10),
      }

      await createEvent(program, eventOrganizer, validEvent)
    })

    it('Should allow buying a ticket successfully', async () => {
      const initialBuyerBalance = await program.provider.connection.getBalance(buyer1.publicKey)
      const [eventPda] = getEventPda(program, validEvent.name, eventOrganizer.publicKey)
      const initialEventBalance = await program.provider.connection.getBalance(eventPda)

      const ticket = {
        event: eventPda,
        buyer: buyer1.publicKey,
        price: validEvent.ticketPrice,
      }

      await buyTicket(program, buyer1, ticket)

      // Verify ticket was created
      await verifyTicket(program, ticket)

      // Verify available tickets decreased
      const eventAccount = await program.account.event.fetch(eventPda)
      assert.equal(eventAccount.availableTickets.toString(), '9')

      // Verify payment was transferred
      const finalBuyerBalance = await program.provider.connection.getBalance(buyer1.publicKey)
      const finalEventBalance = await program.provider.connection.getBalance(eventPda)

      // Buyer should have less SOL (ticket price + transaction fees)
      assert(finalBuyerBalance < initialBuyerBalance - validEvent.ticketPrice.toNumber())
      // Event should have more SOL
      assert(finalEventBalance > initialEventBalance)
    })

    it('Should allow multiple users to buy tickets', async () => {
      const [eventPda] = getEventPda(program, validEvent.name, eventOrganizer.publicKey)
      const ticket1 = {
        event: eventPda,
        buyer: buyer1.publicKey,
        price: validEvent.ticketPrice,
      }
      const ticket2 = {
        event: eventPda,
        buyer: buyer2.publicKey,
        price: validEvent.ticketPrice,
      }
      await buyTicket(program, buyer1, ticket1)
      await buyTicket(program, buyer2, ticket2)

      const eventAccount = await program.account.event.fetch(eventPda)
      assert.equal(eventAccount.availableTickets.toString(), '8') // Started with 10, bought 2
    })

    it('Should fail when trying to buy ticket for event that already started', async () => {
      // Create an event that starts 5 seconds in the future
      const currentTime = Math.floor(Date.now() / 1000)
      const futureEventName = `Future${Date.now().toString().slice(-6)}`
      const futureEvent = {
        name: futureEventName,
        description: 'Future Event',
        startDate: new BN(currentTime + 5), // 5 seconds from now
        ticketPrice: new BN(0.1 * LAMPORTS_PER_SOL),
        availableTickets: new BN(10),
      }

      // Create the event (this should succeed)
      await createEvent(program, eventOrganizer, futureEvent)
      const [eventPda] = getEventPda(program, futureEvent.name, eventOrganizer.publicKey)

      // Wait 7 seconds to ensure the event has started
      console.log('Waiting 7 seconds for event to start...')
      await new Promise((resolve) => setTimeout(resolve, 7000))

      // Try to buy a ticket (this should fail with StartDateInThePast error)
      const ticket = {
        event: eventPda,
        buyer: buyer1.publicKey,
        price: futureEvent.ticketPrice,
      }

      try {
        await buyTicket(program, buyer1, ticket)
        assert(false)
      } catch (error) {
        assert(
          error.message.includes('Start date is in the past'),
          'Should have failed because event has already started',
        )
      }
    }, 15000) // 15 second timeout to allow for the 7 second wait

    it('Should fail when all tickets are sold out', async () => {
      // Create event with only 1 ticket
      const limitedEventName = `Ltd${Date.now().toString().slice(-7)}`
      const limitedEvent = {
        name: limitedEventName,
        description: 'Limited Event',
        startDate: new BN(Math.floor(Date.now() / 1000) + 3600),
        ticketPrice: new BN(0.1 * LAMPORTS_PER_SOL),
        availableTickets: new BN(1),
      }

      await createEvent(program, eventOrganizer, limitedEvent)
      const [eventPda] = getEventPda(program, limitedEvent.name, eventOrganizer.publicKey)

      // Buy the only ticket
      const ticket1 = {
        event: eventPda,
        buyer: buyer1.publicKey,
        price: limitedEvent.ticketPrice,
      }
      await buyTicket(program, buyer1, ticket1)

      // Try to buy another ticket
      const ticket2 = {
        event: eventPda,
        buyer: buyer2.publicKey,
        price: limitedEvent.ticketPrice,
      }
      try {
        await buyTicket(program, buyer2, ticket2)
        assert(false)
      } catch (error) {
        assert(error.message.includes('All tickets sold out'), 'Should have failed because all tickets are sold out')
      }
    })

    it('Should fail when buyer tries to buy the same ticket twice', async () => {
      const [eventPda] = getEventPda(program, validEvent.name, eventOrganizer.publicKey)
      const ticket = {
        event: eventPda,
        buyer: buyer1.publicKey,
        price: validEvent.ticketPrice,
      }

      await buyTicket(program, buyer1, ticket)

      try {
        await buyTicket(program, buyer1, ticket)
        assert(false)
      } catch (error) {
        assert(error.message.includes('already in use'), 'Should have failed because the ticket PDA already exists')
      }
    })

    it('Should fail when buyer does not have enough funds to buy ticket', async () => {
      const poorBuyer = anchor.web3.Keypair.generate()

      const [eventPda] = getEventPda(program, validEvent.name, eventOrganizer.publicKey)
      const ticket = {
        event: eventPda,
        buyer: poorBuyer.publicKey,
        price: validEvent.ticketPrice,
      }

      try {
        await buyTicket(program, poorBuyer, ticket)
        assert(false)
      } catch (error) {
        assert(
          error.message.includes('Transfer: insufficient lamports'),
          'Should have failed because buyer does not have enough funds',
        )
      }
    })
  })

  describe('Withdraw Funds', () => {
    let eventWithFunds: TicketRegistryEvent
    let eventPda: PublicKey

    beforeEach(async () => {
      // Create event and sell some tickets to have funds to withdraw
      const eventName = `Fund${Date.now().toString().slice(-8)}`
      eventWithFunds = {
        name: eventName,
        description: 'Event with funds',
        startDate: new BN(Math.floor(Date.now() / 1000) + 3600),
        ticketPrice: new BN(0.2 * LAMPORTS_PER_SOL),
        availableTickets: new BN(10),
      }

      await createEvent(program, eventOrganizer, eventWithFunds)
      eventPda = getEventPda(program, eventWithFunds.name, eventOrganizer.publicKey)[0]

      // Buy some tickets to add funds
      const ticket1 = {
        event: eventPda,
        buyer: buyer1.publicKey,
        price: eventWithFunds.ticketPrice,
      }
      const ticket2 = {
        event: eventPda,
        buyer: buyer2.publicKey,
        price: eventWithFunds.ticketPrice,
      }
      await buyTicket(program, buyer1, ticket1)
      await buyTicket(program, buyer2, ticket2)
    })

    it('Should allow event organizer to withdraw funds', async () => {
      const initialOrganizerBalance = await program.provider.connection.getBalance(eventOrganizer.publicKey)
      const initialEventBalance = await program.provider.connection.getBalance(eventPda)

      const withdrawAmount = new BN(0.1 * LAMPORTS_PER_SOL)

      await withdrawFunds(program, eventOrganizer, eventWithFunds, withdrawAmount)

      const finalOrganizerBalance = await program.provider.connection.getBalance(eventOrganizer.publicKey)
      const finalEventBalance = await program.provider.connection.getBalance(eventPda)

      // Organizer should have more SOL (minus transaction fees)
      assert(finalOrganizerBalance > initialOrganizerBalance)
      // Event should have less SOL
      assert(finalEventBalance < initialEventBalance)
    })

    it('Should fail when non-organizer tries to withdraw funds', async () => {
      const withdrawAmount = new BN(0.1 * LAMPORTS_PER_SOL)

      // Use the correct event PDA (created by eventOrganizer) but try to withdraw with unauthorizedUser
      const [eventPda] = getEventPda(program, eventWithFunds.name, eventOrganizer.publicKey)

      try {
        await program.methods
          .withdraw(withdrawAmount)
          .accounts({
            eventOrganizer: unauthorizedUser.publicKey,
            event: eventPda,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([unauthorizedUser])
          .rpc()
        assert(false)
      } catch (error) {
        // This should fail due to the has_one constraint in the instruction
        assert(
          error.message.includes('A has one constraint was violated'),
          'Should have failed due to unauthorized withdrawal',
        )
      }
    })

    it('Should fail when trying to withdraw more than available balance', async () => {
      const eventBalance = await program.provider.connection.getBalance(eventPda)
      const withdrawAmount = new BN(eventBalance + LAMPORTS_PER_SOL) // More than available

      try {
        await withdrawFunds(program, eventOrganizer, eventWithFunds, withdrawAmount)
        assert(false)
      } catch (error) {
        // This should fail due to insufficient lamports (arithmetic overflow)
        assert(
          error.message.includes('arithmetic overflowed') || error.message.includes('ArithmeticOverflow'),
          'Should have failed due to insufficient balance',
        )
      }
    })

    it('Should allow partial withdrawal', async () => {
      const initialEventBalance = await program.provider.connection.getBalance(eventPda)
      const withdrawAmount = new BN(0.05 * LAMPORTS_PER_SOL) // Small amount

      await withdrawFunds(program, eventOrganizer, eventWithFunds, withdrawAmount)

      const finalEventBalance = await program.provider.connection.getBalance(eventPda)
      assert(finalEventBalance < initialEventBalance)
      assert(finalEventBalance > 0) // Should still have some funds left
    })
  })
})

// HELPER TYPES AND FUNCTIONS

interface TicketRegistryEvent {
  name: string
  description: string
  startDate: BN
  ticketPrice: BN
  availableTickets: BN
}

interface Ticket {
  event: PublicKey
  buyer: PublicKey
  price: BN
}

async function airdrop(connection: Connection, address: PublicKey, amount: number = 1000 * LAMPORTS_PER_SOL) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), 'confirmed')
}

// INSTRUCTIONS

async function createEvent(
  program: Program<Ticketregistry>,
  eventOrganizer: anchor.web3.Keypair,
  event: TicketRegistryEvent,
) {
  const [eventPda] = getEventPda(program, event.name, eventOrganizer.publicKey)

  await program.methods
    .initialize(event.name, event.description, event.ticketPrice, event.availableTickets, event.startDate)
    .accounts({
      eventOrganizer: eventOrganizer.publicKey,
      event: eventPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([eventOrganizer])
    .rpc()
}

async function buyTicket(program: Program<Ticketregistry>, buyer: anchor.web3.Keypair, ticket: Ticket) {
  const [ticketPda] = getTicketPda(program, ticket.event, ticket.buyer)

  await program.methods
    .buy()
    .accounts({
      buyer: ticket.buyer,
      event: ticket.event,
      ticket: ticketPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([buyer])
    .rpc()
}

async function withdrawFunds(
  program: Program<Ticketregistry>,
  eventOrganizer: anchor.web3.Keypair,
  event: TicketRegistryEvent,
  amount: BN,
) {
  const [eventPda] = getEventPda(program, event.name, eventOrganizer.publicKey)

  await program.methods
    .withdraw(amount)
    .accounts({
      eventOrganizer: eventOrganizer.publicKey,
      event: eventPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([eventOrganizer])
    .rpc()
}

// VERIFY PDAs

async function verifyTicket(program: Program<Ticketregistry>, ticket: Ticket) {
  const [ticketPda] = getTicketPda(program, ticket.event, ticket.buyer)
  const ticketAccount = await program.account.ticket.fetch(ticketPda)

  assert.equal(ticketAccount.event.toString(), ticket.event.toString())
  assert.equal(ticketAccount.buyer.toString(), ticket.buyer.toString())
  assert.equal(ticketAccount.price.toString(), ticket.price.toString())
}

async function verifyEvent(
  program: Program<Ticketregistry>,
  event: TicketRegistryEvent,
  eventOrganizer: anchor.web3.Keypair,
) {
  const [eventPda] = getEventPda(program, event.name, eventOrganizer.publicKey)
  const eventAccount = await program.account.event.fetch(eventPda)

  assert.equal(eventAccount.name, event.name)
  assert.equal(eventAccount.description, event.description)
  assert.equal(eventAccount.startDate.toString(), event.startDate.toString())
  assert.equal(eventAccount.ticketPrice.toString(), event.ticketPrice.toString())
  assert.equal(eventAccount.availableTickets.toString(), event.availableTickets.toString())
  assert.equal(eventAccount.eventOrganizer.toString(), eventOrganizer.publicKey.toString())
}

// GET PDAs

function getEventPda(
  program: Program<Ticketregistry>,
  eventName: string,
  eventOrganizer: PublicKey,
): [PublicKey, number] {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('event'), Buffer.from(eventName), eventOrganizer.toBuffer()],
    program.programId,
  )
}

function getTicketPda(program: Program<Ticketregistry>, eventPda: PublicKey, buyer: PublicKey): [PublicKey, number] {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('ticket'), eventPda.toBuffer(), buyer.toBuffer()],
    program.programId,
  )
}
