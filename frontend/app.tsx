
import React, { useEffect, useState } from "react"
import ReactDOM from "react-dom/client"

const App = () => {
    const [words, setWords] = useState([{german:"",chinese:""}]);
    const [isLoading, setLoading] = useState(true);
    const [germanWord, setGermanWord] = useState();
    const [chineseWord, setChineseWord] = useState();

    const fetchData = () => {
        fetch("http://127.0.0.1:8081/word_pairs")
            .then(response => { return response.json() })
            .then(data => setWords(data))
            .then(_ => setLoading(false))
    };
    useEffect(() => {
        fetchData();
    }, []);

    const handleGermanChange=(event)=>{
        setGermanWord(event.target.value);
    }
    const handleChineseChange=(event)=>{
        setChineseWord(event.target.value);
    }
    const submitHandler=()=>{
        const word ={german:germanWord,chinese:chineseWord};
        fetch("http://127.0.0.1:8081/post_new_words",
        {
            method:"POST",
            headers:{"Content-Type": "application/json"},
            body: JSON.stringify(word),
        })
        .then(response => {return response.json()} )
        .then(data => {
            console.log(data);
            console.log(words.length);
            var copy = [...words];
            copy.push({german: data["german"], chinese: data["chinese"]});
            setWords(copy);
            console.log(words.length);
        });
    }

    if (isLoading) {
        return <div>Loading</div>
    } else {
        return (<div><h1>Word pair list</h1>
            <table>
                <thead>
                    <tr>
                        <th>German</th>
                        <th>Chinese</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        words.map(item => {return  <tr><td>{item.german}</td><td>{item.chinese}</td></tr>; })
                    }
                </tbody>
            </table>
            
            <div id = "enter">
                <p>German</p>
                <input onChange={handleGermanChange}/>
                <p>Chinese</p>
                <input onChange={handleChineseChange}/>
                <button onClick={submitHandler}>submit</button>
            </div>
            
            </div>);
    }
}

const root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
)
