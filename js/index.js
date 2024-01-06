console.log("test");

import init, {lexer,parser} from "../pkg/rode_sql_parser.js";

let wasm = await init();

document.querySelector(".lex").addEventListener("click", () => {
    let query = document.querySelector(".query").value;
    try { 
      let lexedQuery = lexer(query);
      console.log(lexedQuery);

      let lexedString = "";
      lexedQuery.map((token) => {
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

    }catch {
        console.log("idk");
        document.querySelector('.lexed-value').innerHTML = 'Cannot Lex';
    }

    try{
        let parsedQuery = parser(query);
            console.log(parsedQuery);


            let parsedHTMLBuilder = "";
            if (typeof parsedQuery == 'object'){
                let parsedField = Object.values(parsedQuery)[0];
                let parsedName = Object.keys(parsedQuery)[0];
                console.log(parsedName);

                parsedHTMLBuilder += `
                <center>
                    <h3 class="grate">Parsed as ${parsedName}</h3>
                </center>
                `;

                parsedHTMLBuilder += `
                <div class="group">
                ${parsedField.fields.length > 0 ? `<div class="fields">
                    <h3>Fields</h3>
                    <ul>
                    ${parsedField.fields.map((value) => {
                        return `<li class="field">${value}</li>`
                    })}
                    </ul>
                </div>`:``}

                <div class="fields">
                    <h3>Tables</h3>
                    <ul>
                    ${parsedField.tables.map((value) => {
                        return `<li class="field">${value}</li> `
                    })}
                    </ul>
                </div>
                </div>
                `;

                parsedHTMLBuilder += `
                    <h3 class="distinct">Distinct: <span class="grate">${parsedField.is_distinct}</span></h3><hr/>
                `;

                parsedHTMLBuilder +=  parsedField.condition.length > 0 ? `
                    ${parsedField.condition.map((value) => `<h1 class="condition">Condition</h1> 
                    <h3>
                        <span class="grate">${value.left.Assigned}</span> 
                        <span class="grate">${value.operation}</span> 
                        <span class="grate">${value.right.Assigned}</span> 
                    </h3>`)}
                <hr/>` : `some`;
                

                parsedHTMLBuilder += `
                <h3 class="join">Join: 
                ${parsedField.joins.length > 0 ? `
                            ${parsedField.joins.map((values) => {
                                return `<span class="grate"><span class="field">${values.join_type}</span> 
                                on <span class="field">${values.table}</span></span>`
                            })}` : `No Joins Used` }</h3>`;

                
            }else{ 
                parsedHTMLBuilder +=`
                <center>
                    <h3 class="grate">it's valid but I didn't parse for enough information</h3>
                </center>`;
            }



            document.querySelector('.parsed-value').innerHTML = parsedHTMLBuilder;
            }catch{
                document.querySelector('.parsed-value').innerHTML = `
                <center>
                    <h3 class="grate">(Unparsable) Maybe Feature not Implemented</h3>
                </center>
                `;
                // document.querySelector('.lexed-value').innerHTML = '';
            }


    });



// console.log(lexer("select * from test;"));


