console.log("test");

import init, {lexer} from "../pkg/rode_sql_parser.js";

let wasm = await init();

document.querySelector(".lex").addEventListener("click", () => {
    let val = lexer(document.querySelector(".query").value);

    console.log(val);

    let lexedString = "";
    val.map((token) => {
        lexedString += `<tr> 
                            <td>${token.name}</td>
                            <td>${token.group}</td>
                            <td>${token.len}</td>
                            <td>${token.token}</td>
                        </tr>`;
        });

    document.querySelector('.lexed-value').innerHTML = `
                                                <table>
                                                <tr>
                                                    <th>Name</th>
                                                    <th>Group</th>
                                                    <th>Length</th>
                                                    <th>Token</th>
                                                </tr>
                                                ${lexedString}
                                                </table>`;
    });

// console.log(lexer("select * from test;"));


