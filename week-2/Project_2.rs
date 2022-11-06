fn main() {
	let toshiba:f64 = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_000.00; 
	let dell:f64 = 2_850_000.00; 
	let acer:f64 = 250_000.00; 

	let qty_toshiba:f64 = 2.0 ;
	let qtymac:f64 = 1.0;
	let qtyhp:f64  = 3.0;
	let qtydell:f64 = 3.0;
	let qtyacer:f64 = 1.0 ;

	let s = toshiba + mac + hp + dell + acer; 
	println!("The sum is {} ", s );

	let s_qty = qty_toshiba + qtymac + qtyhp + qtydell + qtyacer; 

	let avg = s / s_qty ;

	println!("The Average is {}", avg );  










}