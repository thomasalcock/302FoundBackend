## Requirements

Axum, Axum-Tower, 

## Database

user {
    #id: u64,
    auth_token: String,
    user_name: String,
    full_name: String,
    phone_number: String,
    email: String
    alias: u64
}

trust {
    #truster: u64,
    #trustee: u64
}

location {
    #user: u64,
    #time: date,
    lat: f64,
    long: f64,
    direction: real
}

## API

/auth
    /signup {user_name, password} -> bool
    /login {user_name, password} -> auth_token

/user
    /create {User} -> bool
    /read {} -> [User]
    /read/{u64} -> User
    /update {User} -> bool
    /delete {u64} -> bool

/location
    /create{real, real, real} -> Location
    /read/{u64} -> [Location]
    /read/{u64, date} -> Location
    /delete{u64} -> bool


/trust
    /create {u64} -> bool
    /read -> [User]
    /delete -> bool
