
import React, { useEffect, useState } from "react"
import ReactDOM from "react-dom/client"

const App = () => {
    const [words, setWords] = useState([{ german: "", chinese: "", id: "" }]);
    const [isUpdating, setIsUpdating] = useState({});
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

    const handleGermanChange = (event) => {
        setGermanWord(event.target.value);
    }
    const handleChineseChange = (event) => {
        setChineseWord(event.target.value);
    }
    const submitHandler = () => {
        const word = { german: germanWord, chinese: chineseWord };
        fetch("http://127.0.0.1:8081/word_pairs",
            {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(word),
            })
            .then(response => { return response.json() })
            .then(data => {
                console.log(data);
                console.log(words.length);
                var copy = [...words];
                copy.push({ german: data["german"], chinese: data["chinese"], id: data["id"] });
                setWords(copy);
                console.log(words.length);
            });
    }
    const deleteHandler = (id) => {
        console.log(id);
        const delete_uri = "http://127.0.0.1:8081/word_pairs/" + id;
        fetch(delete_uri,
            {
                method: "DELETE",
                headers: { "Content-Type": "application/json" },
            })
        const list = [...words];
        setWords(list.filter(word => word.id !== id));
    }
    const updateHandler = (item) => {
        if (!isUpdating[item.id]) {
            isUpdating[item.id] = true;
            
            // trigger rendering
            const list = [...words];
            setWords(list);
        } else {
            isUpdating[item.id] = false;
            const german_updated_value = document.getElementById("german-input-" + item.id)?.value;
            const chinese_updated_value = document.getElementById("chinese-input-" + item.id)?.value;

            // update in frontend
            const list = [...words];
            const updated_item = { id: item.id, german: german_updated_value, chinese: chinese_updated_value };
            list[list.indexOf(item)] = updated_item;
            setWords(list);

            // @todo: save new values to database
        }
        console.log(isUpdating);
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
                        words.map(item => {
                            console.log(item);
                            if (isUpdating[item.id]) {
                                return <tr>
                                    <td><div id={"german-" + item.id}><input id={"german-input-" + item.id} defaultValue={item.german} /></div></td>
                                    <td><div id={"chinese-" + item.id}><input id={"chinese-input-" + item.id} defaultValue={item.chinese} /></div></td>
                                    <td><button onClick={() => { updateHandler(item); }} id={"update-" + item.id}>Save</button></td>
                                    <td><button onClick={() => { deleteHandler(item.id); }}>Delete</button></td>
                                </tr>;
                            }
                            else {
                                return <tr>
                                    <td><div id={"german-" + item.id}>{item.german}</div></td>
                                    <td><div id={"chinese-" + item.id}>{item.chinese}</div></td>
                                    <td><button onClick={() => { updateHandler(item); }} id={"update-" + item.id}>Update</button></td>
                                    <td><button onClick={() => { deleteHandler(item.id); }}>Delete</button></td>
                                </tr>;
                            }
                        })
                    }
                </tbody>
            </table>

            <div id="add">
                <p>German</p>
                <input onChange={handleGermanChange} />
                <p>Chinese</p>
                <input onChange={handleChineseChange} />
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
