import { useState } from 'react'
import { to_roman, from_roman } from 'n-roman'
import './App.css'

function App() {
  // State for the input,
  // the input is the value of the input field
  const [input, setInput] = useState<string>('')

  // Try to convert the input to a number
  const num = Number(input)

  // If the input is a number, convert it to roman
  // else try to convert the input to a number

  const result = Number(input)
    ? to_roman(Number(input))
    : from_roman(input.toUpperCase())

  return (
    <>
      <h1>n-roman</h1>
      <div className="card">
        {/* Take the input of the user, try to convert to string or number */}
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
        />
        {/* If the input is a number, show the roman number */}
        {num ? (
          <p>
            {num} in roman is <strong>{result}</strong>
          </p>
        ) : (
          // If the input is not a number, show the number
          <p>
            {input} in number is <strong>{result}</strong>
          </p>
        )}
      </div>
    </>
  )
}

export default App
