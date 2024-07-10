const API_URL = 'http://localhost:3001';

export const send_graph = async (result_width, result_height, graph) => {
    // Jsonify the graph
    let dto = {
        "info": {  "width": result_width, "height": result_height},
        "graph": graph
    }
    let json = JSON.stringify(dto);
    console.log(json);
}
