//enumerateur pour categoriser les produits
pub enum Categorie {
    Boison,
    Electronique,
    Meuble,
    Nothing
}

// Structure de donnees pour les produits
pub struct Product {
    pub name: String,
    pub price: u32,
    pub categorie: Categorie
}

//implementation des methodes et fonction dans la structure de donnees
impl Product{
    //create une nouvelle donnee avec le type Produit
    pub fn add(name: String, price: u32, categorie: Categorie) -> Product {
        Product{
            name,
            price,
            categorie
        }
    }

    pub fn copy_data(&self) -> Product {
        Product{
            name: self.name.clone(),
            price: self.price,
            categorie: match self.categorie {
                Categorie::Boison => Categorie::Boison,
                Categorie::Electronique => Categorie::Electronique,
                Categorie::Meuble => Categorie::Meuble,
                Categorie::Nothing => Categorie::Nothing
            }
        }
    }
}

pub mod stock{
    use super::{Product, Categorie};
    //Une fonction permettant aux personnes de retirer des produits du stok et ainsi
    //les retirer du vecteur all_produit et qui sera ajouter auto dans de_produit
    pub fn un_stock<'a>(index: usize, all_product: &'a mut Vec<Product>, de_product: &'a mut Vec<Product>) -> &'a Vec <Product>{
        let new_product = all_product[index].copy_data();
        de_product.push(new_product);
        all_product.remove(index);
        de_product
    }

    //Une fonction permettant aux personnes d'ajouter de nouveaux produits dans la boutique,
    // qui sera stoker dans le vecteur all_produit
    pub fn stocked<'a>(name: String, price: u32, categorie: Categorie, all_product: &mut Vec<Product>) -> &Vec<Product> {
        let product = Product::add(name, price, categorie);
        all_product.push(product);
        all_product
    }

    pub fn diplay_product(produits: &mut Vec<Product>) {
        for i in 0..produits.len() {
            println!("-----------------------------------------");
            println!("Numero du produit: {}", i);
            println!("Nom du produit: {}", produits[i].name);
            println!("Prix du produit: {}", produits[i].price);
            match produits[i].categorie {
                Categorie::Meuble => println!("Categorie du produit: Meuble"),
                Categorie::Electronique => println!("Categorie du produit: Electronique"),
                Categorie::Boison => println!("Categorie du produit: Boison"),
                Categorie::Nothing => println!("La categorie n'hesite pas")
            }
            println!("-----------------------------------------");
        }
    }
}
