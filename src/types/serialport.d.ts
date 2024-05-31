type serialPortInfo = {
    
    port_name: string,
    port_type: {
        USBPort: {
            vid: number,
            pid: number,
            serial_number: string,
            manufacturer: string,
            product: string
        }
    }
}[]