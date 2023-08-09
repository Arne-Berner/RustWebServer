public interface ICommandHandler<TCommand>
{
    void Handle(TCommand command);
}

public class MoveCustomerCommandHandler   //Exactly the same as before,
    : ICommandHandler<MoveCustomerCommand>{
    private readonly UnitOfWork db;

    public MoveCustomerCommandHandler(
        UnitOfWork db) //Could have more dependencies
    {
        this.db = db;
    }
 
    public void Handle(MoveCustomerCommand command)
    {
        // TODO: Logic here
    }
}

public class CustomerController : Controller
{
    private ICommandHandler<MoveCustomerCommand> handler;
 
    public CustomerController(         
        ICommandHandler<MoveCustomerCommand> handler)
    {
        this.handler = handler;
    }
 
    public void MoveCustomer(int customerId, Address newAddress)
    {
        var command = new MoveCustomerCommand
        {
            CustomerId = customerId,
            NewAddress = newAddress
        };
 
        this.handler.Handle(command);
    }
}