import { LoginService } from "./login.service";

describe("LoginService", () => {
    let service: LoginService;

    beforeEach(() => {
        service = new LoginService();
    });

    it("should be created", () => {
        expect(service).toBeTruthy();
    });
});
