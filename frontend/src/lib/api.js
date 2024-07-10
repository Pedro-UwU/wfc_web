const API_URL = 'http://localhost:3001';

export const generate_dto = (result_width, result_height, graph) => {
    let dto = {
        info: { "width": result_width, "height": result_height},
        graph: graph
    }
    console.log(dto);
    return dto 
}
