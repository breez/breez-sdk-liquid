const readline = require('readline')

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: true
})

const confirm = (message) => {
    return new Promise((resolve) => {
        rl.question(`${message} (y/n): `, (answer) => {
            resolve(answer.toLowerCase() === 'y' || answer.toLowerCase() === 'yes')
        })
    })
}

const prompt = (prompt) => {
    return new Promise((resolve) => {
        rl.question(`${prompt} > `, (command) => {
            resolve(command)
        })
    })
}

const question = (message, parser) => {
    return new Promise((resolve, reject) => {
        rl.question(`${message}: `, (answer) => {
            if (answer.length === 0) {
                reject()
            } else {
                if (parser) {
                    try {
                        resolve(parser(answer))
                    } catch (e) {
                        reject()
                    }
                } else {
                    resolve(answer)
                }
            }
        })
    })
}

module.exports = { confirm, prompt, question }
