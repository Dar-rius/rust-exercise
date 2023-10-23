use std::io;
pub use project_learn::{stock, Categorie, Product};

fn main(){
    let mut all_produit : Vec<Product> = Vec::new();
    let mut de_produit : Vec<Product> = Vec::new();
    println!("Votre programme de stockage prefere");
    loop{
        let mut choice  = String::new();
        let mut name = String::new();
        let mut price = String::new();
        let mut second_choice = String::new();
        let mut cat: Categorie = Categorie::Nothing;
        let mut name_product = String::new();
        println!("Faite votre choix entre \n a - Ajouter \n b - Retirer \n c - Voir l'ensemble des donnees");
        io::stdin().read_line(&mut choice).expect("Un probleme sur l'entree");
        let choice = choice.trim();
        match choice {
            _ if choice == "a" =>{
                println!("Entrer le nom du produit");
                io::stdin().read_line(&mut name).expect("Un probleme sur l'entree");

                println!("Entrer le prix du produit");
                io::stdin().read_line(&mut price).expect("Un probleme sur l'entree");
                let price : u32 = price.trim().parse().expect("La conversion est impossible");

                println!("Entrer le categirie du produit entre :");
                println!("1 - Boison \n2 - Electronique \n3 - Meuble ");
                io::stdin().read_line(&mut second_choice).expect("Un probleme sur l'entree");
                let second_choice = second_choice.trim();
                match second_choice {
                    _ if second_choice == "1" => cat = Categorie::Boison,
                    _ if second_choice == "2" => cat = Categorie::Electronique,
                    _ if second_choice == "3" => cat = Categorie::Meuble,
                    _ => println!("Error sur le choix")
                }
                stock::stocked(name, price, cat, &mut all_produit);
            }
            _ if choice == "b" =>{
                println!("Entrer le nom du produit:");
                io::stdin().read_line(&mut name_product).expect("Un probleme sur l'entree");

                for i in 0..all_produit.len(){
                    if all_produit[i].name == name_product {
                        stock::un_stock(i, &mut all_produit, &mut de_produit);
                        println!("Ce produit a ete retire");
                    } else{
                        println!("Ce produit n'a pas ete trouve");
                    }
                }
            }
            _ if choice == "c" => {
                if all_produit.len() > 0 || de_produit.len() > 0{
                    println!("nombre de produit en stock: {} ",all_produit.len());
                    stock::diplay_product(&mut all_produit);
                    println!("nombre de produit en destock: {} \n",de_produit.len());
                    stock::diplay_product(&mut de_produit);
                }
                else {
                    println!("Il existe aucun produit");
                }
            }
            _ => break
        }
    }
}