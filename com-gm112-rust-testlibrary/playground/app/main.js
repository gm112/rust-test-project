import { add } from 'com-gm112-rust-testlibrary'
const result = add(2, 2)

console.log('Hello, World!')
console.log('2 + 2 =', result)

const result_element = document.createElement('p')
result_element.textContent = `2 + 2 = ${result}`
document.body.appendChild(result_element)

const first_input_element = document.createElement('input')
first_input_element.type = 'text'
first_input_element.placeholder = 'First number'
first_input_element.value = '2'
document.body.appendChild(first_input_element)

const second_input_element = document.createElement('input')
second_input_element.type = 'text'
second_input_element.placeholder = 'Second number'
second_input_element.value = '2'
document.body.appendChild(second_input_element)

const submit_button = document.createElement('button')
submit_button.textContent = 'Submit'
submit_button.addEventListener('click', () => {
  const first_number = parseInt(first_input_element.value)
  const second_number = parseInt(second_input_element.value)
  const result = add(first_number, second_number)
  result_element.textContent = `${first_number} + ${second_number} = ${result}`
})

document.body.appendChild(submit_button)
