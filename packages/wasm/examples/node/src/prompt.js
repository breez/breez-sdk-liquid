const confirm = (rl, message) => {
    return new Promise((resolve) => {
        rl.question(`${message} (y/n): `, (answer) => {
            resolve(answer.toLowerCase() === 'y' || answer.toLowerCase() === 'yes')
        })
    })
}

const command = (rl, prompt) => {
    return new Promise((resolve) => {
        rl.question(`${prompt} > `, (command) => {
            resolve(command)
        })
    })
}

module.exports = {confirm, command}