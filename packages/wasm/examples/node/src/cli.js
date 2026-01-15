const { Command, Option, Argument } = require('commander')
const { prompt } = require('./prompt.js')
const {
    buyBitcoin,
    checkMessage,
    disconnect,
    getInfo,
    getPayment,
    listFiat,
    listPayments,
    listRefundables,
    lnurlAuth,
    lnurlPay,
    lnurlWithdraw,
    fetchFiatRates,
    fetchLightningLimits,
    fetchOnchainLimits,
    parse,
    prepareRefund,
    receivePayment,
    recommendedFees,
    refund,
    registerWebhook,
    rescanOnchainSwaps,
    reviewPaymentProposedFees,
    sendOnchainPayment,
    sendPayment,
    signMessage,
    sync,
    unregisterWebhook,
    addNwcConnection,
    listNwcConnections,
    removeNwcConnection,
} = require('./action.js')
const { parse: parseShell } = require('shell-quote')

const initCommand = () => {
    const program = new Command()
    program.exitOverride()
    program.name('nodeless-wasm-cli').description('CLI for Breez SDK - Nodeless Wasm')

    program
        .command('buy-bitcoin')
        .description('Generates an URL to buy bitcoin from a 3rd party provider')
        .addArgument(new Argument('<provider>', 'The provider to use').choices(['moonpay']))
        .addArgument(new Argument('<amount-sat>', 'Amount to buy, in satoshi'))
        .action(buyBitcoin)

    program
        .command('check-message')
        .description('Verify a message with a public key')
        .addArgument(new Argument('<message>', 'The message signed'))
        .addArgument(new Argument('<pubkey>', 'The pubkey to verify against'))
        .addArgument(new Argument('<signature>', 'The signature to verify'))
        .action(checkMessage)

    program.command('disconnect').alias('exit').description('Exit the CLI').action(disconnect)

    program.command('get-info').description('Get the balance and general info of the current instance').action(getInfo)

    program
        .command('get-payment')
        .description('Retrieve a payment')
        .addOption(new Option('-p, --payment-hash <string>', 'Lightning payment hash'))
        .addOption(new Option('-s, --swap-id <string>', 'Swap ID or its hash'))
        .action(getPayment)

    program.command('fetch-fiat-rates').description('Fetch available fiat rates').action(fetchFiatRates)

    program
        .command('fetch-lightning-limits')
        .description('Fetch the current limits for Send and Receive payments')
        .action(fetchLightningLimits)

    program
        .command('fetch-onchain-limits')
        .description('Fetch the current limits for Onchain Send and Receive payments')
        .action(fetchOnchainLimits)

    program.command('list-fiat').description('List fiat currencies').action(listFiat)

    program
        .command('list-payments')
        .description('List incoming and outgoing payments')
        .addOption(new Option('-r, --filter <choice>', 'The optional payment type filter').choices(['send', 'receive']))
        .addOption(
            new Option('-s, --state <choice>', 'The optional payment state').choices([
                'pending',
                'complete',
                'failed',
                'pendingRefund',
                'refundable'
            ])
        )
        .addOption(new Option('-f, --from <number>', 'The optional from unix timestamp').argParser(parseInt))
        .addOption(new Option('-t, --to <number>', 'The optional to unix timestamp').argParser(parseInt))
        .addOption(new Option('-l, --limit <number>', 'Optional limit of listed payments').argParser(parseInt))
        .addOption(new Option('-o, --offset <number>', 'Optional offset of listed payments').argParser(parseInt))
        .addOption(new Option('--asset <string>', 'Optional id of the asset for Liquid payment method'))
        .addOption(
            new Option('-d, --destination <string>', 'Optional Liquid BIP21 URI/address for Liquid payment method')
        )
        .addOption(new Option('-a, --address <string>', 'Optional Liquid/Bitcoin address for Bitcoin payment method'))
        .addOption(new Option('--ascending', 'Whether or not to sort the payments by ascending timestamp'))
        .action(listPayments)

    program.command('list-refundables').description('List refundable chain swaps').action(listRefundables)

    program
        .command('lnurl-auth')
        .description('Authorize using LNURL')
        .addArgument(new Argument('<lnurl>', 'LNURL-auth encoded endpoint'))
        .action(lnurlAuth)

    program
        .command('lnurl-pay')
        .description('Pay using LNURL')
        .addArgument(new Argument('<lnurl>', 'LNURL-pay encoded endpoint'))
        .addOption(
            new Option(
                '-d, --drain',
                'Whether or not this is a drain operation. If true, all available funds will be used'
            )
        )
        .addOption(new Option('-v, --validate', 'Validates the success action URL'))
        .action(lnurlPay)

    program
        .command('lnurl-withdraw')
        .description('Withdraw using LNURL')
        .addArgument(new Argument('<lnurl>', 'LNURL-withdraw encoded endpoint'))
        .action(lnurlWithdraw)

    program
        .command('parse')
        .description('Parse a generic string to get its type and relevant metadata')
        .addArgument(new Argument('<input>', 'Generic input (URL, LNURL, BIP-21 Bitcoin Address, LN invoice, etc)'))
        .action(parse)

    program
        .command('prepare-refund')
        .description('Prepare a refund transaction for an incomplete swap')
        .addArgument(new Argument('<swap-address>', 'Swap address of the lockup'))
        .addArgument(new Argument('<refund-address>', 'Bitcoin onchain address to send the refund to'))
        .addArgument(new Argument('<fee-rate>', 'Fee rate to use, in sat/vbyte'))
        .action(prepareRefund)

    program
        .command('receive-payment')
        .description('Receive a payment directly or via a swap')
        .addOption(
            new Option('-m, --payment-method <choice>', 'The method to use when receiving')
                .makeOptionMandatory(true)
                .choices(['lightning', 'bitcoinAddress', 'liquidAddress'])
        )
        .addOption(
            new Option(
                '--amount-sat <number>',
                'The amount the payer should send, in satoshi. If not specified, it will generate a BIP21 URI/address with no amount'
            ).argParser(parseInt)
        )
        .addOption(
            new Option(
                '--asset <string>',
                'Optional id of the asset to receive when the payment method is "liquidAddress"'
            )
        )
        .addOption(
            new Option(
                '--amount <number>',
                'The amount the payer should send, in asset units. If not specified, it will generate a BIP21 URI/address with no amount. The asset id must also be provided'
            ).argParser(parseFloat)
        )
        .action(receivePayment)

    program.command('recommended-fees').description('Get the recommended Bitcoin fees').action(recommendedFees)

    program
        .command('refund')
        .description('Broadcast a refund transaction for an incomplete swap')
        .addArgument(new Argument('<swap-address>', 'Swap address of the lockup'))
        .addArgument(new Argument('<refund-address>', 'Bitcoin onchain address to send the refund to'))
        .addArgument(new Argument('<fee-rate>', 'Fee rate to use, in sat/vbyte'))
        .action(refund)

    program
        .command('register-webhook')
        .description('Register a webhook URL')
        .addArgument(new Argument('<url>', 'The URL to register'))
        .action(registerWebhook)

    program.command('rescan-onchain-swaps').description('Rescan onchain swaps').action(rescanOnchainSwaps)

    program
        .command('review-payment-proposed-fees')
        .description('Get and potentially accept proposed fees for WaitingFeeAcceptance Payment')
        .addArgument(new Argument('<swap-id>', 'The swap id of the payment to review'))
        .action(reviewPaymentProposedFees)

    program
        .command('send-onchain-payment')
        .description('Send to a Bitcoin onchain address via a swap')
        .addArgument(new Argument('<address>', 'Bitcoin onchain address to send to'))
        .addOption(
            new Option(
                '--receiver-amount-sat <number>',
                'Amount that will be received, in satoshi. Must be set if not draining'
            ).argParser(parseInt)
        )
        .addOption(
            new Option(
                '-d, --drain',
                'Whether or not this is a drain operation. If true, all available funds will be used'
            )
        )
        .addOption(
            new Option('-f, --fee-rate <number>', 'The optional fee rate to use, in sat/vbyte').argParser(parseInt)
        )
        .action(sendOnchainPayment)

    program
        .command('send-payment')
        .description('Send a payment directly or via a swap')
        .addOption(new Option('-i, --invoice <string>', 'Invoice which has to be paid'))
        .addOption(new Option('-o, --offer <string>', 'BOLT12 offer. If specified, amount-sat must also be set'))
        .addOption(new Option('-a, --address <string>', 'Either BIP21 URI or Liquid address we intend to pay to'))
        .addOption(
            new Option(
                '--amount-sat <number>',
                'The amount the payer should send, in satoshi. If not specified, it will generate a BIP21 URI/address with no amount'
            ).argParser(parseInt)
        )
        .addOption(
            new Option(
                '--asset <string>',
                'Optional id of the asset to receive when the payment method is "liquidAddress"'
            )
        )
        .addOption(
            new Option(
                '--amount <number>',
                'The amount the payer should send, in asset units. If not specified, it will generate a BIP21 URI/address with no amount. The asset id must also be provided'
            ).argParser(parseFloat)
        )
        .addOption(
            new Option(
                '-d, --drain',
                'Whether or not this is a drain operation. If true, all available funds will be used'
            )
        )
        .addOption(new Option('--use-asset-fees', 'Whether or not the tx should be paid using the asset'))
        .addOption(
            new Option(
                '--from-asset <string>',
                'The asset id specifying which asset to use to execute the payment.'
            )
        )
        .action(sendPayment)

    program
        .command('sign-message')
        .description('Sign a message using the wallet private key')
        .addArgument(new Argument('<message>', 'The message to sign'))
        .action(signMessage)

    program.command('sync').description('Sync local data with mempool and onchain data').action(sync)

    program.command('unregister-webhook').description('Unregister the webhook URL').action(unregisterWebhook)

    const nwc = program.command('nwc').description('Run NWC features')
    nwc
        .command("add-connection")
        .description("Adds an NWC connection")
        .addArgument(new Argument('<name>', 'The unique identifier of the connection'))
        .addOption(new Option('--receive-only', 'Whether the connection is receive-only'))
        .addOption(new Option('--expiry <number>', 'The expiry time of the connection, in minutes').argParser(parseInt))
        .addOption(new Option('--period-time <number>', 'The duration of a periodic budget, in minutes').argParser(parseInt))
        .addOption(new Option('--max-budget <number>', 'The maximum budget for a period').argParser(parseInt))
        .action(addNwcConnection)

    nwc
        .command("list-connections")
        .description("Lists active NWC connections")
        .action(listNwcConnections)

    nwc
        .command("remove-connection")
        .description("Removes an active NWC connection")
        .addArgument(new Argument('<name>', 'The unique identifier of the connection'))
        .action(removeNwcConnection)

    return program
}

const main = () => {
    return new Promise(async (resolve) => {
        while (true) {
            try {
                const res = await prompt('sdk')
                if (res.trim().toLowerCase() === 'exit') {
                    disconnect()
                    resolve()
                    break
                } else {
                    const cmd = res.length > 0 ? res : '-h'
                    const program = initCommand()
                    await program.parseAsync(parseShell(cmd), { from: 'user' })
                }
            } catch (e) {
                if (!e.code) {
                    console.error('Error:', e)
                }
            }
        }
    })
}

main()
