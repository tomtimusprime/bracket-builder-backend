-- create database brackets_db;
use brackets_db;

/*Bracket Table*/
CREATE TABLE IF NOT EXISTS Brackets (
    id INT AUTO_INCREMENT PRIMARY KEY,
    topic VARCHAR(255) NOT NULL,
    size INT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

/*Bracket Items Table*/
CREATE TABLE IF NOT EXISTS bracket_items (
    id INT AUTO_INCREMENT PRIMARY KEY,
    bracket_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    FOREIGN KEY (bracket_id) REFERENCES Brackets(id)
);

/*Bracket Rounds Table*/
CREATE TABLE IF NOT EXISTS bracket_rounds (
    bracket_id INT NOT NULL,
    item1_id INT NOT NULL,
    item2_id INT NOT NULL,
    round_id INT NOT NULL,
    position INT NOT NULL,
    FOREIGN KEY (bracket_id) REFERENCES Brackets(id),
    FOREIGN KEY (item1_id) REFERENCES bracket_items(id),
    FOREIGN KEY (item2_id) REFERENCES bracket_items(id),
    PRIMARY KEY (bracket_id, round_id, position)
);

/*Bracket Votes Table*/
CREATE TABLE IF NOT EXISTS bracket_votes (
    bracket_id INT NOT NULL,
    item_id INT NOT NULL,
    round_id INT NOT NULL,
    FOREIGN KEY (bracket_id) REFERENCES Brackets(id),
    FOREIGN KEY (item_id) REFERENCES bracket_items(id),
    PRIMARY KEY (bracket_id, item_id, round_id)
);

-- INSERT INTO Brackets (topic, size, created_at) VALUES ('Favorite Fruits', 8, '2024-01-15 10:00:00');

-- SELECT * from Brackets;
