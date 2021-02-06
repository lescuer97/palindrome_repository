 export function searcher(a:number): number[] | string {
    // array for stocking palindromes
    const palindromes: number[] = [];
    //loops every number
    for (let i = 10; i < a + 1  ; i++) {
        const arrOfNumber: string[] = i.toString().split(''); 
        const length = arrOfNumber.length - 1
        
        //if both ends don't match don't operate reverse
        if (arrOfNumber[0] === arrOfNumber[length]) {

            const notReversed = arrOfNumber.join('');
            const reverse = arrOfNumber.reverse().join('');

            // console.log("Normal: ",notReversed, "reverse: ", reverse );

            // if reverse match normalizr data and send to array
            if(notReversed === reverse) {

                const normalizedNumber: number = +reverse;
                
                palindromes.push(normalizedNumber);
            }
        }    
    }

    if (palindromes.length === 0){
    return "There are no Palindromes"
    }

return palindromes;
}

export default searcher;