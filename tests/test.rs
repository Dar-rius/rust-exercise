pub use project_learn::{Product, Categorie};

fn val(name: String, price: u32, cat: Categorie) -> Product{
    Product::add(name, price, cat)
}

fn panic_test(produit: &mut Vec<Product>, index: usize){
    println!("{}", produit[index].name);
}

#[cfg(test)]
mod testing {
    use super::{val};
    pub use project_learn::{Product, Categorie, stock};
    use crate::panic_test;

    #[test]
    fn create_product() {
        let mut all_produits: Vec<Product> = Vec::new();
        let produit = val("test".to_string(), 12000, Categorie::Meuble);
        stock::stocked("test".to_string(), 12000, Categorie::Meuble, &mut all_produits);
        assert_eq!(all_produits[0].name, produit.name)
    }

    #[test]
    fn delete_product() {
        let mut all_product: Vec<Product> = Vec::new();
        let mut de_product: Vec<Product> = Vec::new();
        let produit = val("test".to_string(), 12000, Categorie::Meuble);
        stock::stocked("test".to_string(), 12000, Categorie::Meuble, &mut all_product);
        stock::un_stock(0, &mut all_product, &mut de_product);
        assert_eq!(de_product[0].name, produit.name);
    }

    #[test]
    #[should_panic]
    fn panic_product() {
        let mut all_product: Vec<Product> = Vec::new();
        let mut de_product: Vec<Product> = Vec::new();
        stock::stocked("test".to_string(), 12000, Categorie::Meuble, &mut all_product);
        stock::un_stock(0, &mut all_product, &mut de_product);
        panic_test(&mut all_product, 0);
    }
}