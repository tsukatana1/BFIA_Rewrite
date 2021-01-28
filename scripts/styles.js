class Radio {
    constructor(element) {
        if(!typeof element == HTMLDivElement) {
            return new Error("Invalid element. Please pass in a div where the radio buttons are located at.");
        }

        this.parentElement = element
        
        const radioButtons = document.querySelectorAll(`.${this.parentElement.name} input[type="radio"]`);

        if(radioButtons == undefined) {
            return new Error('No radio buttons detected.');
        }

        mapElements(this);

    }
}

function mapElements(buttons) {
    return Array.from(buttons.radioButtons).map(radio => {
        
    })
}