import React from "react";


interface CardProps{
    title: string;
    description?: string;
    image?: string | undefined;
}

const classes = ['card', 'card-active'];

function Card(props: CardProps) {
    const img = props.image;
    const desc = props.description;
    
    const [presence, setPresence] = React.useState(false);

    const [currentClass, setCurrentClass] = React.useState(classes[0]);

    const copyToClip = () => {
        // toast "Coppied to clipboard"
        navigator.clipboard.writeText(desc || props.title);
    }

    const togglePresence = () => {
        // toast "Presence toggled"
        setPresence(!presence);
        
        setCurrentClass(presence ? classes[0] : classes[1]);
    }
    
    return (
        <div className={`${currentClass}`} onClick={togglePresence}>
            {img && <img src={img} alt="Card Image" />}
            <h2>{props.title}</h2>
            {desc && <p>{desc}</p>}
        </div>
    )
}




export default Card