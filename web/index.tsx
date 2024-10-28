import React, { useState } from 'react';
import { createRoot } from 'react-dom/client';

import Card from './components/card';

function App() {
    const [numbers, setNumbers] = useState([]);
    const [count, setCount] = useState(0);

    const fetchRandomNumbers = async () => {
        const res = await fetch("/api/random/" + count, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            },
            body: JSON.stringify({min:1, max:100})
        });
        const data = await res.json();
        setNumbers(data);
    }
    
    

    return (
        <div className='app'>
            <div className='field-inp-btn'>
                <input type='number' value={count} onChange={(e) => setCount(parseInt(e.target.value))} />
                <button onClick={fetchRandomNumbers}>Fetch</button>
            </div>

            <div className='card-list'>
                {numbers.map((number, index) => {
                    return <Card key={index} title={`Card ${index + 1}`} description={`${number}`} />
                })}
            </div>
            
        </div>
    )
}




const container = document.getElementById('App') || document.getElementById('app');
const root = createRoot(container!);

root.render(<App />);