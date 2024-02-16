use std::collections::HashSet;

fn main() {
// Hashset
  //let user_ids: HashSet<i32> = vec![11, 123, 443];
  let mut user_ids = HashSet::new();
  user_ids.insert(100);
  user_ids.insert(900);
  user_ids.insert(2);

// union: obtener los elementos únicos entre 2 sets
// difference: obtener los elementos que están en el primer set, y no en el otro
// intersection: obtener sólo los elementos que están en ambos sets.
// symetric_difference: obtener todos los elementos que están en un set, o en el otro, 
// pero no en ambos.
  let mut backup_users = HashSet::new();
  backup_users.insert(100);
  backup_users.insert(900);
  backup_users.insert(23);
  backup_users.insert(9);

  for id in user_ids.difference(&backup_users) {
    println!("{}", id);
  }
}
